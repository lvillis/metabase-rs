use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Jitter {
    None,
    Full,
}

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub(crate) max_retries: usize,
    pub(crate) base_delay: Duration,
    pub(crate) max_delay: Duration,
    pub(crate) jitter: Jitter,
}

impl RetryPolicy {
    pub fn disabled() -> Self {
        Self {
            max_retries: 0,
            base_delay: Duration::from_secs(0),
            max_delay: Duration::from_secs(0),
            jitter: Jitter::None,
        }
    }

    pub fn conservative() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(2),
            jitter: Jitter::Full,
        }
    }

    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn base_delay(mut self, base_delay: Duration) -> Self {
        self.base_delay = base_delay;
        self
    }

    pub fn max_delay(mut self, max_delay: Duration) -> Self {
        self.max_delay = max_delay;
        self
    }

    pub fn jitter(mut self, jitter: Jitter) -> Self {
        self.jitter = jitter;
        self
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::conservative()
    }
}
