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
pub(crate) struct AsyncTransport {
    client: reqwest::Client,
    capture_body_snippet: bool,
    body_snippet_limit: usize,
    redact_body_snippet: bool,
    retry: crate::client::RetryPolicy,
}

impl AsyncTransport {
    pub(crate) fn new(config: &ClientConfig) -> Result<Self, Error> {
        let _ = rustls::crypto::ring::default_provider().install_default();

        let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        let client = reqwest::Client::builder()
            .connect_timeout(config.timeouts.connect)
            .timeout(config.timeouts.request)
            .user_agent(user_agent)
            .build()
            .map_err(Error::transport)?;

        Ok(Self {
            client,
            capture_body_snippet: config.body_snippet.capture,
            body_snippet_limit: config.body_snippet.limit,
            redact_body_snippet: config.body_snippet.redact,
            retry: config.retry.clone(),
        })
    }

    pub(crate) async fn execute_json<T, B>(
        &self,
        method: Method,
        url: Url,
        auth: &Auth,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
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

            let mut request = self
                .client
                .request(method.clone(), url.clone())
                .headers(headers.clone());

            if let Some(timeout) = options.timeout {
                request = request.timeout(timeout);
            }

            if let Some(body) = body {
                request = request.json(body);
            }

            let response = match request.send().await {
                Ok(response) => response,
                Err(err) => {
                    if can_retry
                        && retry_count < self.retry.max_retries
                        && is_retryable_reqwest_error(&err)
                    {
                        let delay = super::retry::next_delay(&self.retry, retry_count);
                        retry_count += 1;
                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
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
                    return Err(Error::transport_with_context(method, path, err));
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
                let _ = response.bytes().await;
                if !delay.is_zero() {
                    tokio::time::sleep(delay).await;
                }
                continue;
            }

            let request_id = util::extract_request_id(&response_headers);

            let body_bytes = response
                .bytes()
                .await
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

    pub(crate) async fn execute_bytes<B>(
        &self,
        method: Method,
        url: Url,
        auth: &Auth,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<Vec<u8>, Error>
    where
        B: Serialize + ?Sized,
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

            let mut request = self
                .client
                .request(method.clone(), url.clone())
                .headers(headers.clone());

            if let Some(timeout) = options.timeout {
                request = request.timeout(timeout);
            }

            if let Some(body) = body {
                request = request.json(body);
            }

            let response = match request.send().await {
                Ok(response) => response,
                Err(err) => {
                    if can_retry
                        && retry_count < self.retry.max_retries
                        && is_retryable_reqwest_error(&err)
                    {
                        let delay = super::retry::next_delay(&self.retry, retry_count);
                        retry_count += 1;
                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
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
                    return Err(Error::transport_with_context(method, path, err));
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
                let _ = response.bytes().await;
                if !delay.is_zero() {
                    tokio::time::sleep(delay).await;
                }
                continue;
            }

            let request_id = util::extract_request_id(&response_headers);

            let body_bytes = response
                .bytes()
                .await
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

                return Ok(body_bytes.to_vec());
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

    pub(crate) async fn execute_multipart_json<T>(
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

            let mut request = self
                .client
                .request(method.clone(), url.clone())
                .headers(headers.clone());

            if let Some(timeout) = options.timeout {
                request = request.timeout(timeout);
            }

            let mut reqwest_form = reqwest::multipart::Form::new();
            for (name, value) in form.fields() {
                reqwest_form = reqwest_form.text(name.clone(), value.clone());
            }
            for file in form.files() {
                let mut part = reqwest::multipart::Part::bytes(file.bytes().to_vec())
                    .file_name(file.filename().to_owned());
                if let Some(content_type) = file.content_type() {
                    part = part.mime_str(content_type).map_err(|err| {
                        Error::transport_with_context(method.clone(), path.clone(), err)
                    })?;
                }
                reqwest_form = reqwest_form.part(file.name().to_owned(), part);
            }
            request = request.multipart(reqwest_form);

            let response = match request.send().await {
                Ok(response) => response,
                Err(err) => {
                    if can_retry
                        && retry_count < self.retry.max_retries
                        && is_retryable_reqwest_error(&err)
                    {
                        let delay = super::retry::next_delay(&self.retry, retry_count);
                        retry_count += 1;
                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
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
                    return Err(Error::transport_with_context(method, path, err));
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
                let _ = response.bytes().await;
                if !delay.is_zero() {
                    tokio::time::sleep(delay).await;
                }
                continue;
            }

            let request_id = util::extract_request_id(&response_headers);

            let body_bytes = response
                .bytes()
                .await
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
}

fn is_retryable_reqwest_error(err: &reqwest::Error) -> bool {
    err.is_connect() || err.is_timeout()
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
