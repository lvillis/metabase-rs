use http::{HeaderMap, Method, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use url::Url;

#[cfg(feature = "metrics")]
use std::time::Instant;

use crate::{
    auth::Auth,
    client::{ClientConfig, RequestOptions},
    error::{ApiError, DecodeError, Error},
    types::multipart::MultipartForm,
    util,
};

#[derive(Clone)]
pub(crate) struct BlockingTransport {
    agent: ureq::Agent,
    capture_body_snippet: bool,
    body_snippet_limit: usize,
    redact_body_snippet: bool,
    retry: crate::client::RetryPolicy,
}

impl BlockingTransport {
    pub(crate) fn new(config: &ClientConfig) -> Result<Self, Error> {
        let _ = rustls::crypto::ring::default_provider().install_default();

        let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        let capture_body_snippet = config.body_snippet.capture;
        let body_snippet_limit = config.body_snippet.limit;

        let agent_config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .user_agent(user_agent)
            .timeout_connect(Some(config.timeouts.connect))
            .timeout_global(Some(config.timeouts.request))
            .timeout_recv_body(Some(config.timeouts.read))
            .build();

        Ok(Self {
            agent: ureq::Agent::new_with_config(agent_config),
            capture_body_snippet,
            body_snippet_limit,
            redact_body_snippet: config.body_snippet.redact,
            retry: config.retry.clone(),
        })
    }

    pub(crate) fn execute_json<T, B>(
        &self,
        method: Method,
        url: Url,
        auth: &Auth,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let path = url.path().to_owned();
        let can_retry = super::retry::is_idempotent_method(&method)
            || (method == Method::POST && options.idempotency_key.is_some());

        let mut headers = HeaderMap::new();
        auth.apply(&mut headers)?;
        headers.insert("Accept", http::HeaderValue::from_static("application/json"));
        if let Some(key) = &options.idempotency_key {
            let value = http::HeaderValue::from_str(key.as_ref())
                .map_err(|source| Error::invalid_header_value("Idempotency-Key", source))?;
            headers.insert("Idempotency-Key", value);
        }

        let mut retry_count = 0usize;
        loop {
            #[cfg(feature = "tracing")]
            let _span = tracing::info_span!(
                "metabase.request",
                method = %method,
                path = %path,
                retry_count = retry_count
            )
            .entered();

            #[cfg(feature = "metrics")]
            metrics::counter!(
                "metabase_request_attempts_total",
                "method" => method_label(&method)
            )
            .increment(1);

            #[cfg(feature = "metrics")]
            let started_at = Instant::now();

            let response = match self.execute(
                method.clone(),
                url.clone(),
                headers.clone(),
                body,
                options.timeout,
            ) {
                Ok(response) => response,
                Err(err) => {
                    if can_retry
                        && retry_count < self.retry.max_retries
                        && is_retryable_ureq_error(&err)
                    {
                        let delay = super::retry::next_delay(&self.retry, retry_count);
                        retry_count += 1;
                        if !delay.is_zero() {
                            std::thread::sleep(delay);
                        }
                        continue;
                    }
                    #[cfg(feature = "metrics")]
                    metrics::counter!(
                        "metabase_requests_total",
                        "method" => method_label(&method),
                        "outcome" => "transport_error"
                    )
                    .increment(1);
                    #[cfg(feature = "metrics")]
                    metrics::histogram!(
                        "metabase_request_duration_seconds",
                        "method" => method_label(&method),
                        "outcome" => "transport_error"
                    )
                    .record(started_at.elapsed().as_secs_f64());
                    return Err(Error::transport_with_context(
                        method.clone(),
                        path.clone(),
                        err,
                    ));
                }
            };

            let status: StatusCode = response.status();
            let response_headers = response.headers().clone();

            if can_retry
                && retry_count < self.retry.max_retries
                && super::retry::is_retryable_status(status)
            {
                let delay = super::retry::retry_after(&response_headers)
                    .unwrap_or_else(|| super::retry::next_delay(&self.retry, retry_count));
                retry_count += 1;
                let _ = response.into_body().read_to_vec();
                if !delay.is_zero() {
                    std::thread::sleep(delay);
                }
                continue;
            }

            let request_id = util::extract_request_id(&response_headers);

            let body_bytes = response
                .into_body()
                .read_to_vec()
                .map_err(|err| Error::transport_with_context(method.clone(), path.clone(), err))?;

            if status.is_success() {
                let body_slice = if body_bytes.iter().all(|b| b.is_ascii_whitespace()) {
                    b"null".as_slice()
                } else {
                    body_bytes.as_ref()
                };
                let mut deserializer = serde_json::Deserializer::from_slice(body_slice);
                let parsed: std::result::Result<T, serde_path_to_error::Error<serde_json::Error>> =
                    serde_path_to_error::deserialize(&mut deserializer);

                let result = parsed.map_err(|err| {
                    let snippet = self.capture_body_snippet.then(|| {
                        util::capture_body_snippet(
                            &body_bytes,
                            self.body_snippet_limit,
                            self.redact_body_snippet,
                        )
                    });
                    Error::Decode(DecodeError::new(
                        status,
                        method.clone(),
                        path.clone(),
                        request_id,
                        Some(err.path().to_string()),
                        err.into_inner(),
                        snippet,
                    ))
                });

                #[cfg(feature = "metrics")]
                metrics::counter!(
                    "metabase_requests_total",
                    "method" => method_label(&method),
                    "outcome" => if result.is_ok() { "ok" } else { "decode_error" }
                )
                .increment(1);
                #[cfg(feature = "metrics")]
                metrics::histogram!(
                    "metabase_request_duration_seconds",
                    "method" => method_label(&method),
                    "outcome" => if result.is_ok() { "ok" } else { "decode_error" }
                )
                .record(started_at.elapsed().as_secs_f64());

                return result;
            }

            let body_json: Option<Value> = serde_json::from_slice(&body_bytes).ok();
            let message = body_json
                .as_ref()
                .and_then(|v| v.get("message"))
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned);

            let retry_after = super::retry::retry_after(&response_headers);
            let snippet = self.capture_body_snippet.then(|| {
                util::capture_body_snippet(
                    &body_bytes,
                    self.body_snippet_limit,
                    self.redact_body_snippet,
                )
            });

            let err = Error::from_api_error(
                ApiError::new(
                    status,
                    method.clone(),
                    path.clone(),
                    request_id,
                    message,
                    body_json,
                    snippet,
                ),
                retry_after,
            );

            #[cfg(feature = "metrics")]
            metrics::counter!(
                "metabase_requests_total",
                "method" => method_label(&method),
                "outcome" => status_class(status)
            )
            .increment(1);
            #[cfg(feature = "metrics")]
            metrics::histogram!(
                "metabase_request_duration_seconds",
                "method" => method_label(&method),
                "outcome" => status_class(status)
            )
            .record(started_at.elapsed().as_secs_f64());

            return Err(err);
        }
    }

    pub(crate) fn execute_bytes<B>(
        &self,
        method: Method,
        url: Url,
        auth: &Auth,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<Vec<u8>, Error>
    where
        B: Serialize,
    {
        let path = url.path().to_owned();
        let can_retry = super::retry::is_idempotent_method(&method)
            || (method == Method::POST && options.idempotency_key.is_some());

        let mut headers = HeaderMap::new();
        auth.apply(&mut headers)?;
        headers.insert("Accept", http::HeaderValue::from_static("*/*"));
        if let Some(key) = &options.idempotency_key {
            let value = http::HeaderValue::from_str(key.as_ref())
                .map_err(|source| Error::invalid_header_value("Idempotency-Key", source))?;
            headers.insert("Idempotency-Key", value);
        }

        let mut retry_count = 0usize;
        loop {
            #[cfg(feature = "tracing")]
            let _span = tracing::info_span!(
                "metabase.request",
                method = %method,
                path = %path,
                retry_count = retry_count
            )
            .entered();

            #[cfg(feature = "metrics")]
            metrics::counter!(
                "metabase_request_attempts_total",
                "method" => method_label(&method)
            )
            .increment(1);

            #[cfg(feature = "metrics")]
            let started_at = Instant::now();

            let response = match self.execute(
                method.clone(),
                url.clone(),
                headers.clone(),
                body,
                options.timeout,
            ) {
                Ok(response) => response,
                Err(err) => {
                    if can_retry
                        && retry_count < self.retry.max_retries
                        && is_retryable_ureq_error(&err)
                    {
                        let delay = super::retry::next_delay(&self.retry, retry_count);
                        retry_count += 1;
                        if !delay.is_zero() {
                            std::thread::sleep(delay);
                        }
                        continue;
                    }
                    #[cfg(feature = "metrics")]
                    metrics::counter!(
                        "metabase_requests_total",
                        "method" => method_label(&method),
                        "outcome" => "transport_error"
                    )
                    .increment(1);
                    #[cfg(feature = "metrics")]
                    metrics::histogram!(
                        "metabase_request_duration_seconds",
                        "method" => method_label(&method),
                        "outcome" => "transport_error"
                    )
                    .record(started_at.elapsed().as_secs_f64());
                    return Err(Error::transport_with_context(
                        method.clone(),
                        path.clone(),
                        err,
                    ));
                }
            };

            let status: StatusCode = response.status();
            let response_headers = response.headers().clone();

            if can_retry
                && retry_count < self.retry.max_retries
                && super::retry::is_retryable_status(status)
            {
                let delay = super::retry::retry_after(&response_headers)
                    .unwrap_or_else(|| super::retry::next_delay(&self.retry, retry_count));
                retry_count += 1;
                let _ = response.into_body().read_to_vec();
                if !delay.is_zero() {
                    std::thread::sleep(delay);
                }
                continue;
            }

            let request_id = util::extract_request_id(&response_headers);

            let body_bytes = response
                .into_body()
                .read_to_vec()
                .map_err(|err| Error::transport_with_context(method.clone(), path.clone(), err))?;

            if status.is_success() {
                #[cfg(feature = "metrics")]
                metrics::counter!(
                    "metabase_requests_total",
                    "method" => method_label(&method),
                    "outcome" => "ok"
                )
                .increment(1);
                #[cfg(feature = "metrics")]
                metrics::histogram!(
                    "metabase_request_duration_seconds",
                    "method" => method_label(&method),
                    "outcome" => "ok"
                )
                .record(started_at.elapsed().as_secs_f64());

                return Ok(body_bytes);
            }

            let body_json: Option<Value> = serde_json::from_slice(&body_bytes).ok();
            let message = body_json
                .as_ref()
                .and_then(|v| v.get("message"))
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned);

            let retry_after = super::retry::retry_after(&response_headers);
            let snippet = self.capture_body_snippet.then(|| {
                util::capture_body_snippet(
                    &body_bytes,
                    self.body_snippet_limit,
                    self.redact_body_snippet,
                )
            });

            let err = Error::from_api_error(
                ApiError::new(
                    status,
                    method.clone(),
                    path.clone(),
                    request_id,
                    message,
                    body_json,
                    snippet,
                ),
                retry_after,
            );

            #[cfg(feature = "metrics")]
            metrics::counter!(
                "metabase_requests_total",
                "method" => method_label(&method),
                "outcome" => status_class(status)
            )
            .increment(1);
            #[cfg(feature = "metrics")]
            metrics::histogram!(
                "metabase_request_duration_seconds",
                "method" => method_label(&method),
                "outcome" => status_class(status)
            )
            .record(started_at.elapsed().as_secs_f64());

            return Err(err);
        }
    }

    pub(crate) fn execute_multipart_json<T>(
        &self,
        method: Method,
        url: Url,
        auth: &Auth,
        form: &MultipartForm,
        options: RequestOptions,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let path = url.path().to_owned();
        let can_retry = super::retry::is_idempotent_method(&method)
            || (method == Method::POST && options.idempotency_key.is_some());

        let mut headers = HeaderMap::new();
        auth.apply(&mut headers)?;
        headers.insert("Accept", http::HeaderValue::from_static("application/json"));
        if let Some(key) = &options.idempotency_key {
            let value = http::HeaderValue::from_str(key.as_ref())
                .map_err(|source| Error::invalid_header_value("Idempotency-Key", source))?;
            headers.insert("Idempotency-Key", value);
        }

        let mut retry_count = 0usize;
        loop {
            #[cfg(feature = "tracing")]
            let _span = tracing::info_span!(
                "metabase.request",
                method = %method,
                path = %path,
                retry_count = retry_count
            )
            .entered();

            #[cfg(feature = "metrics")]
            metrics::counter!(
                "metabase_request_attempts_total",
                "method" => method_label(&method)
            )
            .increment(1);

            #[cfg(feature = "metrics")]
            let started_at = Instant::now();

            let response = match self.execute_multipart(
                method.clone(),
                url.clone(),
                headers.clone(),
                form,
                options.timeout,
            ) {
                Ok(response) => response,
                Err(err) => {
                    if can_retry
                        && retry_count < self.retry.max_retries
                        && is_retryable_ureq_error(&err)
                    {
                        let delay = super::retry::next_delay(&self.retry, retry_count);
                        retry_count += 1;
                        if !delay.is_zero() {
                            std::thread::sleep(delay);
                        }
                        continue;
                    }
                    #[cfg(feature = "metrics")]
                    metrics::counter!(
                        "metabase_requests_total",
                        "method" => method_label(&method),
                        "outcome" => "transport_error"
                    )
                    .increment(1);
                    #[cfg(feature = "metrics")]
                    metrics::histogram!(
                        "metabase_request_duration_seconds",
                        "method" => method_label(&method),
                        "outcome" => "transport_error"
                    )
                    .record(started_at.elapsed().as_secs_f64());
                    return Err(Error::transport_with_context(
                        method.clone(),
                        path.clone(),
                        err,
                    ));
                }
            };

            let status: StatusCode = response.status();
            let response_headers = response.headers().clone();

            if can_retry
                && retry_count < self.retry.max_retries
                && super::retry::is_retryable_status(status)
            {
                let delay = super::retry::retry_after(&response_headers)
                    .unwrap_or_else(|| super::retry::next_delay(&self.retry, retry_count));
                retry_count += 1;
                let _ = response.into_body().read_to_vec();
                if !delay.is_zero() {
                    std::thread::sleep(delay);
                }
                continue;
            }

            let request_id = util::extract_request_id(&response_headers);

            let body_bytes = response
                .into_body()
                .read_to_vec()
                .map_err(|err| Error::transport_with_context(method.clone(), path.clone(), err))?;

            if status.is_success() {
                let body_slice = if body_bytes.iter().all(|b| b.is_ascii_whitespace()) {
                    b"null".as_slice()
                } else {
                    body_bytes.as_ref()
                };
                let mut deserializer = serde_json::Deserializer::from_slice(body_slice);
                let parsed: std::result::Result<T, serde_path_to_error::Error<serde_json::Error>> =
                    serde_path_to_error::deserialize(&mut deserializer);

                let result = parsed.map_err(|err| {
                    let snippet = self.capture_body_snippet.then(|| {
                        util::capture_body_snippet(
                            &body_bytes,
                            self.body_snippet_limit,
                            self.redact_body_snippet,
                        )
                    });
                    Error::Decode(DecodeError::new(
                        status,
                        method.clone(),
                        path.clone(),
                        request_id,
                        Some(err.path().to_string()),
                        err.into_inner(),
                        snippet,
                    ))
                });

                #[cfg(feature = "metrics")]
                metrics::counter!(
                    "metabase_requests_total",
                    "method" => method_label(&method),
                    "outcome" => if result.is_ok() { "ok" } else { "decode_error" }
                )
                .increment(1);
                #[cfg(feature = "metrics")]
                metrics::histogram!(
                    "metabase_request_duration_seconds",
                    "method" => method_label(&method),
                    "outcome" => if result.is_ok() { "ok" } else { "decode_error" }
                )
                .record(started_at.elapsed().as_secs_f64());

                return result;
            }

            let body_json: Option<Value> = serde_json::from_slice(&body_bytes).ok();
            let message = body_json
                .as_ref()
                .and_then(|v| v.get("message"))
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned);

            let retry_after = super::retry::retry_after(&response_headers);
            let snippet = self.capture_body_snippet.then(|| {
                util::capture_body_snippet(
                    &body_bytes,
                    self.body_snippet_limit,
                    self.redact_body_snippet,
                )
            });

            let err = Error::from_api_error(
                ApiError::new(
                    status,
                    method.clone(),
                    path.clone(),
                    request_id,
                    message,
                    body_json,
                    snippet,
                ),
                retry_after,
            );

            #[cfg(feature = "metrics")]
            metrics::counter!(
                "metabase_requests_total",
                "method" => method_label(&method),
                "outcome" => status_class(status)
            )
            .increment(1);
            #[cfg(feature = "metrics")]
            metrics::histogram!(
                "metabase_request_duration_seconds",
                "method" => method_label(&method),
                "outcome" => status_class(status)
            )
            .record(started_at.elapsed().as_secs_f64());

            return Err(err);
        }
    }

    fn execute<B: Serialize>(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<&B>,
        timeout_override: Option<std::time::Duration>,
    ) -> Result<http::Response<ureq::Body>, ureq::Error> {
        let url = url.as_str();

        match body {
            None => {
                let mut request = match method {
                    Method::GET => self.agent.get(url),
                    Method::DELETE => self.agent.delete(url),
                    Method::HEAD => self.agent.head(url),
                    Method::OPTIONS => self.agent.options(url),
                    Method::TRACE => self.agent.trace(url),
                    Method::POST | Method::PUT | Method::PATCH => {
                        return Err(ureq::Error::Other(Box::new(std::io::Error::other(
                            "request body required",
                        ))));
                    }
                    _ => {
                        let mut builder = http::Request::builder().method(method).uri(url);
                        for (name, value) in headers.iter() {
                            builder = builder.header(name, value);
                        }
                        let request = builder.body(()).map_err(ureq::Error::from)?;
                        return self.agent.run(request);
                    }
                };

                if let Some(timeout) = timeout_override {
                    request = request.config().timeout_global(Some(timeout)).build();
                }

                for (name, value) in headers.iter() {
                    request = request.header(name, value);
                }

                request.call()
            }
            Some(body) => {
                let mut request = match method {
                    Method::POST => self.agent.post(url),
                    Method::PUT => self.agent.put(url),
                    Method::PATCH => self.agent.patch(url),
                    Method::GET
                    | Method::DELETE
                    | Method::HEAD
                    | Method::OPTIONS
                    | Method::TRACE => {
                        return Err(ureq::Error::Other(Box::new(std::io::Error::other(
                            "unexpected request body",
                        ))));
                    }
                    _ => {
                        let send_body = ureq::SendBody::from_json(body)?;

                        let mut builder = http::Request::builder().method(method).uri(url);
                        for (name, value) in headers.iter() {
                            builder = builder.header(name, value);
                        }

                        let request = builder.body(send_body).map_err(ureq::Error::from)?;
                        return self.agent.run(request);
                    }
                };

                if let Some(timeout) = timeout_override {
                    request = request.config().timeout_global(Some(timeout)).build();
                }

                for (name, value) in headers.iter() {
                    request = request.header(name, value);
                }

                request.send_json(body)
            }
        }
    }

    fn execute_multipart(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        form: &MultipartForm,
        timeout_override: Option<std::time::Duration>,
    ) -> Result<http::Response<ureq::Body>, ureq::Error> {
        use ureq::unversioned::multipart::{Form, Part};

        let url = url.as_str();

        let mut request = match method {
            Method::POST => self.agent.post(url),
            Method::PUT => self.agent.put(url),
            Method::PATCH => self.agent.patch(url),
            _ => {
                return Err(ureq::Error::Other(Box::new(std::io::Error::other(
                    "unsupported multipart request method",
                ))));
            }
        };

        if let Some(timeout) = timeout_override {
            request = request.config().timeout_global(Some(timeout)).build();
        }

        for (name, value) in headers.iter() {
            request = request.header(name, value);
        }

        let mut ureq_form = Form::new();
        for (name, value) in form.fields() {
            ureq_form = ureq_form.text(name.as_str(), value.as_str());
        }
        for file in form.files() {
            let mut part = Part::bytes(file.bytes()).file_name(file.filename());
            if let Some(content_type) = file.content_type() {
                part = part.mime_str(content_type)?;
            }
            ureq_form = ureq_form.part(file.name(), part);
        }

        request.send(ureq_form)
    }
}

fn is_retryable_ureq_error(err: &ureq::Error) -> bool {
    match err {
        ureq::Error::Timeout(_)
        | ureq::Error::HostNotFound
        | ureq::Error::ConnectionFailed
        | ureq::Error::BodyStalled
        | ureq::Error::TlsRequired => true,
        ureq::Error::Io(io) => matches!(
            io.kind(),
            std::io::ErrorKind::ConnectionReset
                | std::io::ErrorKind::ConnectionAborted
                | std::io::ErrorKind::NotConnected
                | std::io::ErrorKind::TimedOut
                | std::io::ErrorKind::BrokenPipe
        ),
        _ => false,
    }
}

#[cfg(feature = "metrics")]
fn status_class(status: StatusCode) -> &'static str {
    if status.is_success() {
        "2xx"
    } else if status.is_client_error() {
        "4xx"
    } else if status.is_server_error() {
        "5xx"
    } else {
        "other"
    }
}

#[cfg(feature = "metrics")]
fn method_label(method: &Method) -> &'static str {
    match *method {
        Method::GET => "GET",
        Method::POST => "POST",
        Method::PUT => "PUT",
        Method::PATCH => "PATCH",
        Method::DELETE => "DELETE",
        Method::HEAD => "HEAD",
        Method::OPTIONS => "OPTIONS",
        _ => "OTHER",
    }
}
