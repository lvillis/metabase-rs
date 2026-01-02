use std::{
    cmp,
    time::{Duration, SystemTime},
};

use http::{HeaderMap, Method, StatusCode};

use crate::client::{Jitter, RetryPolicy};

pub(crate) fn is_retryable_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

pub(crate) fn is_idempotent_method(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS
    )
}

pub(crate) fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get("retry-after")?;
    let value = value.to_str().ok()?;
    let value = value.trim();

    if value.is_empty() {
        return None;
    }

    if let Ok(secs) = value.parse::<u64>() {
        return Some(Duration::from_secs(secs));
    }

    let date = httpdate::parse_http_date(value).ok()?;
    let now = SystemTime::now();
    let delay = date.duration_since(now).unwrap_or(Duration::from_secs(0));
    Some(delay)
}

pub(crate) fn next_delay(policy: &RetryPolicy, attempt: usize) -> Duration {
    if policy.max_retries == 0 {
        return Duration::from_secs(0);
    }

    let shift = cmp::min(attempt, 31);
    let factor = 1u32 << shift;

    let mut delay = policy.base_delay.saturating_mul(factor);
    if delay > policy.max_delay {
        delay = policy.max_delay;
    }

    match policy.jitter {
        Jitter::None => delay,
        Jitter::Full => full_jitter(delay),
    }
}

fn full_jitter(delay: Duration) -> Duration {
    let max_ms = u64::try_from(cmp::min(delay.as_millis(), u128::from(u64::MAX))).unwrap_or(0);
    if max_ms == 0 {
        return Duration::from_secs(0);
    }
    Duration::from_millis(fastrand::u64(0..=max_ms))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn retry_after_parses_seconds() -> Result<(), Box<dyn StdError + Send + Sync>> {
        let mut headers = HeaderMap::new();
        headers.insert("Retry-After", http::HeaderValue::from_static("120"));
        assert_eq!(retry_after(&headers), Some(Duration::from_secs(120)));
        Ok(())
    }

    #[test]
    fn retry_after_parses_http_date() -> Result<(), Box<dyn StdError + Send + Sync>> {
        let mut headers = HeaderMap::new();
        let value = httpdate::fmt_http_date(SystemTime::UNIX_EPOCH);
        headers.insert("Retry-After", http::HeaderValue::from_str(&value)?);
        assert_eq!(retry_after(&headers), Some(Duration::from_secs(0)));
        Ok(())
    }

    #[test]
    fn next_delay_is_exponential_and_clamped_without_jitter() {
        let policy = RetryPolicy::conservative()
            .max_retries(10)
            .base_delay(Duration::from_millis(100))
            .max_delay(Duration::from_millis(500))
            .jitter(Jitter::None);

        assert_eq!(next_delay(&policy, 0), Duration::from_millis(100));
        assert_eq!(next_delay(&policy, 1), Duration::from_millis(200));
        assert_eq!(next_delay(&policy, 2), Duration::from_millis(400));
        assert_eq!(next_delay(&policy, 3), Duration::from_millis(500));
        assert_eq!(next_delay(&policy, 10), Duration::from_millis(500));
    }

    #[test]
    fn retryable_statuses_match_expectations() {
        assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable_status(StatusCode::SERVICE_UNAVAILABLE));
        assert!(!is_retryable_status(StatusCode::BAD_REQUEST));
    }

    #[test]
    fn idempotent_methods_match_expectations() {
        assert!(is_idempotent_method(&Method::GET));
        assert!(is_idempotent_method(&Method::DELETE));
        assert!(!is_idempotent_method(&Method::POST));
    }
}
