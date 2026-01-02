use std::time::Duration;

use crate::util;

use super::RetryPolicy;

#[derive(Clone)]
pub(crate) struct TimeoutConfig {
    pub(crate) connect: Duration,
    pub(crate) request: Duration,
    pub(crate) read: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect: Duration::from_secs(10),
            request: Duration::from_secs(30),
            read: Duration::from_secs(30),
        }
    }
}

#[derive(Clone)]
pub(crate) struct BodySnippetConfig {
    pub(crate) capture: bool,
    pub(crate) limit: usize,
    pub(crate) redact: bool,
}

impl Default for BodySnippetConfig {
    fn default() -> Self {
        Self {
            capture: true,
            limit: util::DEFAULT_BODY_SNIPPET_LIMIT,
            redact: true,
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct ClientConfig {
    pub(crate) timeouts: TimeoutConfig,
    pub(crate) body_snippet: BodySnippetConfig,
    pub(crate) retry: RetryPolicy,
}
