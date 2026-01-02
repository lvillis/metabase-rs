use std::time::Duration;

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct RequestOptions {
    pub(crate) timeout: Option<Duration>,
    pub(crate) idempotency_key: Option<IdempotencyKey>,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn idempotency_key(mut self, key: IdempotencyKey) -> Self {
        self.idempotency_key = Some(key);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for IdempotencyKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for IdempotencyKey {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
