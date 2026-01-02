use std::{fmt, time::Duration};

use http::{Method, StatusCode};

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("invalid base url: {base_url}")]
    InvalidBaseUrl {
        base_url: String,
        #[source]
        source: url::ParseError,
    },

    #[error("invalid base url: {base_url} ({reason})")]
    InvalidBaseUrlConfig {
        base_url: String,
        reason: &'static str,
    },

    #[error("failed to build request url")]
    BuildUrl {
        #[source]
        source: url::ParseError,
    },

    #[error("failed to serialize query parameters")]
    SerializeQuery {
        #[source]
        source: BoxError,
    },

    #[error("invalid header value for {header}")]
    InvalidHeaderValue {
        header: &'static str,
        #[source]
        source: http::header::InvalidHeaderValue,
    },

    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error("auth error: {0}")]
    Auth(ApiError),

    #[error("not found: {0}")]
    NotFound(ApiError),

    #[error("conflict: {0}")]
    Conflict(ApiError),

    #[error("rate limited: {0}")]
    RateLimited(RateLimitedError),

    #[error(transparent)]
    Api(ApiError),

    #[error(transparent)]
    Decode(DecodeError),
}

impl Error {
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Error::Auth(err) | Error::NotFound(err) | Error::Conflict(err) | Error::Api(err) => {
                Some(err.status())
            }
            Error::RateLimited(err) => Some(err.status()),
            Error::Decode(err) => Some(err.status()),
            _ => None,
        }
    }

    pub fn method(&self) -> Option<&Method> {
        match self {
            Error::Transport(err) => err.method(),
            Error::Auth(err) | Error::NotFound(err) | Error::Conflict(err) | Error::Api(err) => {
                Some(err.method())
            }
            Error::RateLimited(err) => Some(err.method()),
            Error::Decode(err) => Some(err.method()),
            _ => None,
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self {
            Error::Transport(err) => err.path(),
            Error::Auth(err) | Error::NotFound(err) | Error::Conflict(err) | Error::Api(err) => {
                Some(err.path())
            }
            Error::RateLimited(err) => Some(err.path()),
            Error::Decode(err) => Some(err.path()),
            _ => None,
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        match self {
            Error::Auth(err) | Error::NotFound(err) | Error::Conflict(err) | Error::Api(err) => {
                err.request_id()
            }
            Error::RateLimited(err) => err.request_id(),
            Error::Decode(err) => err.request_id(),
            _ => None,
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Error::Auth(err) | Error::NotFound(err) | Error::Conflict(err) | Error::Api(err) => {
                err.message()
            }
            Error::RateLimited(err) => err.message(),
            _ => None,
        }
    }

    pub fn body_snippet(&self) -> Option<&str> {
        match self {
            Error::Auth(err) | Error::NotFound(err) | Error::Conflict(err) | Error::Api(err) => {
                err.body_snippet()
            }
            Error::RateLimited(err) => err.body_snippet(),
            Error::Decode(err) => err.body_snippet(),
            _ => None,
        }
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Error::RateLimited(err) => err.retry_after(),
            _ => None,
        }
    }

    pub(crate) fn invalid_base_url(base_url: impl Into<String>, source: url::ParseError) -> Self {
        Self::InvalidBaseUrl {
            base_url: base_url.into(),
            source,
        }
    }

    pub(crate) fn invalid_base_url_config(
        base_url: impl Into<String>,
        reason: &'static str,
    ) -> Self {
        Self::InvalidBaseUrlConfig {
            base_url: base_url.into(),
            reason,
        }
    }

    pub(crate) fn build_url(source: url::ParseError) -> Self {
        Self::BuildUrl { source }
    }

    pub(crate) fn serialize_query<E>(source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::SerializeQuery {
            source: Box::new(source),
        }
    }

    pub(crate) fn invalid_header_value(
        header: &'static str,
        source: http::header::InvalidHeaderValue,
    ) -> Self {
        Self::InvalidHeaderValue { header, source }
    }

    #[cfg(feature = "async")]
    pub(crate) fn transport<E>(source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TransportError::new(None, None, source).into()
    }

    pub(crate) fn transport_with_context<E>(method: Method, path: String, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TransportError::new(Some(method), Some(path), source).into()
    }

    pub(crate) fn from_api_error(err: ApiError, retry_after: Option<Duration>) -> Self {
        match err.status() {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Auth(err),
            StatusCode::NOT_FOUND => Error::NotFound(err),
            StatusCode::CONFLICT | StatusCode::PRECONDITION_FAILED => Error::Conflict(err),
            StatusCode::TOO_MANY_REQUESTS => {
                Error::RateLimited(RateLimitedError::new(err, retry_after))
            }
            _ => Error::Api(err),
        }
    }
}

pub struct TransportError {
    method: Option<Method>,
    path: Option<String>,
    source: BoxError,
}

impl TransportError {
    pub(crate) fn new<E>(method: Option<Method>, path: Option<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            method,
            path,
            source: Box::new(source),
        }
    }

    pub fn method(&self) -> Option<&Method> {
        self.method.as_ref()
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
}

impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportError")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("source", &"<redacted>")
            .finish()
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.method, &self.path) {
            (Some(method), Some(path)) => {
                write!(f, "transport error (method={method}, path={path})")
            }
            _ => write!(f, "transport error"),
        }
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

pub struct ApiError {
    status: StatusCode,
    method: Method,
    path: String,
    request_id: Option<String>,
    message: Option<String>,
    body: Option<serde_json::Value>,
    body_snippet: Option<String>,
}

impl ApiError {
    pub(crate) fn new(
        status: StatusCode,
        method: Method,
        path: String,
        request_id: Option<String>,
        message: Option<String>,
        body: Option<serde_json::Value>,
        body_snippet: Option<String>,
    ) -> Self {
        Self {
            status,
            method,
            path,
            request_id,
            message,
            body,
            body_snippet,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn body(&self) -> Option<&serde_json::Value> {
        self.body.as_ref()
    }

    pub fn body_snippet(&self) -> Option<&str> {
        self.body_snippet.as_deref()
    }
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("status", &self.status)
            .field("method", &self.method)
            .field("path", &self.path)
            .field("request_id", &self.request_id)
            .field("message", &self.message)
            .field("body", &redacted_option(self.body.as_ref()))
            .field(
                "body_snippet",
                &redacted_option(self.body_snippet.as_deref()),
            )
            .finish()
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "api error (status={}, method={}, path={}",
            self.status, self.method, self.path
        )?;
        if let Some(request_id) = &self.request_id {
            write!(f, ", request_id={request_id}")?;
        }
        if let Some(message) = &self.message {
            write!(f, ", message={message}")?;
        }
        write!(f, ")")
    }
}

impl std::error::Error for ApiError {}

pub struct RateLimitedError {
    inner: ApiError,
    retry_after: Option<Duration>,
}

impl RateLimitedError {
    pub(crate) fn new(inner: ApiError, retry_after: Option<Duration>) -> Self {
        Self { inner, retry_after }
    }

    pub fn status(&self) -> StatusCode {
        self.inner.status()
    }

    pub fn method(&self) -> &Method {
        self.inner.method()
    }

    pub fn path(&self) -> &str {
        self.inner.path()
    }

    pub fn request_id(&self) -> Option<&str> {
        self.inner.request_id()
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message()
    }

    pub fn body(&self) -> Option<&serde_json::Value> {
        self.inner.body()
    }

    pub fn body_snippet(&self) -> Option<&str> {
        self.inner.body_snippet()
    }

    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }
}

impl fmt::Debug for RateLimitedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RateLimitedError")
            .field("inner", &self.inner)
            .field("retry_after", &self.retry_after)
            .finish()
    }
}

impl fmt::Display for RateLimitedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.retry_after {
            None => write!(f, "{}", self.inner),
            Some(duration) => write!(f, "{} (retry_after={:?})", self.inner, duration),
        }
    }
}

impl std::error::Error for RateLimitedError {}

pub struct DecodeError {
    status: StatusCode,
    method: Method,
    path: String,
    request_id: Option<String>,
    serde_path: Option<String>,
    source: serde_json::Error,
    body_snippet: Option<String>,
}

impl DecodeError {
    pub(crate) fn new(
        status: StatusCode,
        method: Method,
        path: String,
        request_id: Option<String>,
        serde_path: Option<String>,
        source: serde_json::Error,
        body_snippet: Option<String>,
    ) -> Self {
        Self {
            status,
            method,
            path,
            request_id,
            serde_path,
            source,
            body_snippet,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn serde_path(&self) -> Option<&str> {
        self.serde_path.as_deref()
    }

    pub fn source(&self) -> &serde_json::Error {
        &self.source
    }

    pub fn body_snippet(&self) -> Option<&str> {
        self.body_snippet.as_deref()
    }
}

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DecodeError")
            .field("status", &self.status)
            .field("method", &self.method)
            .field("path", &self.path)
            .field("request_id", &self.request_id)
            .field("serde_path", &self.serde_path)
            .field("source", &self.source)
            .field(
                "body_snippet",
                &redacted_option(self.body_snippet.as_deref()),
            )
            .finish()
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "decode error (status={}, method={}, path={}",
            self.status, self.method, self.path
        )?;
        if let Some(request_id) = &self.request_id {
            write!(f, ", request_id={request_id}")?;
        }
        if let Some(path) = &self.serde_path {
            write!(f, ", serde_path={path}")?;
        }
        write!(f, ", source={}", self.source)?;
        write!(f, ")")
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

fn redacted_option<T>(value: Option<T>) -> Option<&'static str> {
    value.map(|_| "<redacted>")
}
