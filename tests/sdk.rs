#![cfg(all(feature = "async", feature = "mock"))]

use std::error::Error as StdError;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};

use http::StatusCode;
use metabase::{Auth, Client, Error, IdempotencyKey, Jitter, RequestOptions, RetryPolicy};
use wiremock::{
    Mock, MockServer, Request, Respond, ResponseTemplate,
    matchers::{header, method, path},
};

type TestResult<T> = std::result::Result<T, Box<dyn StdError + Send + Sync>>;

#[derive(Clone)]
struct SequenceResponder {
    calls: Arc<AtomicUsize>,
    first: ResponseTemplate,
    later: ResponseTemplate,
}

impl SequenceResponder {
    fn new(first: ResponseTemplate, later: ResponseTemplate) -> Self {
        Self {
            calls: Arc::new(AtomicUsize::new(0)),
            first,
            later,
        }
    }
}

impl Respond for SequenceResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        if call == 0 {
            self.first.clone()
        } else {
            self.later.clone()
        }
    }
}

#[tokio::test]
async fn health_get_ok() -> TestResult<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok"
        })))
        .mount(&server)
        .await;

    let client = Client::builder(server.uri())?.build()?;
    let health = client.health().get().await?;
    assert_eq!(health.status, "ok");
    Ok(())
}

#[tokio::test]
async fn user_current_sends_session_header() -> TestResult<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/user/current"))
        .and(header("x-metabase-session", "TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1
        })))
        .mount(&server)
        .await;

    let client = Client::builder(server.uri())?
        .auth(Auth::session("TOKEN"))
        .build()?;
    let user = client.user().get_current().await?;
    assert_eq!(user.get("id").and_then(|v| v.as_i64()), Some(1));
    Ok(())
}

#[tokio::test]
async fn api_error_exposes_status_and_message() -> TestResult<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "message": "boom"
        })))
        .mount(&server)
        .await;

    let client = Client::builder(server.uri())?.build()?;

    let err = match client.health().get().await {
        Ok(_) => {
            return Err(std::io::Error::other("expected error").into());
        }
        Err(err) => err,
    };

    let Error::Api(api) = err else {
        return Err(std::io::Error::other("unexpected error type").into());
    };

    assert_eq!(api.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(api.message(), Some("boom"));
    assert!(api.body_snippet().is_some());
    Ok(())
}

#[tokio::test]
async fn decode_error_exposes_status_and_body_snippet() -> TestResult<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let client = Client::builder(server.uri())?.build()?;

    let err = match client.health().get().await {
        Ok(_) => {
            return Err(std::io::Error::other("expected error").into());
        }
        Err(err) => err,
    };

    let Error::Decode(decode) = err else {
        return Err(std::io::Error::other("unexpected error type").into());
    };

    assert_eq!(decode.status(), StatusCode::OK);
    assert!(decode.body_snippet().is_some());
    Ok(())
}

#[tokio::test]
async fn retries_429_retry_after_seconds() -> TestResult<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(SequenceResponder::new(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "0")
                .set_body_json(serde_json::json!({ "message": "slow down" })),
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })),
        ))
        .mount(&server)
        .await;

    let retry_policy = RetryPolicy::conservative()
        .max_retries(1)
        .base_delay(Duration::from_secs(1))
        .max_delay(Duration::from_secs(1))
        .jitter(Jitter::None);

    let client = Client::builder(server.uri())?
        .retry_policy(retry_policy)
        .build()?;

    let health = tokio::time::timeout(Duration::from_millis(500), client.health().get()).await??;
    assert_eq!(health.status, "ok");

    let requests = server
        .received_requests()
        .await
        .ok_or_else(|| std::io::Error::other("wiremock did not record requests"))?;
    assert_eq!(requests.len(), 2);
    Ok(())
}

#[tokio::test]
async fn retries_429_retry_after_http_date() -> TestResult<()> {
    let server = MockServer::start().await;

    let retry_after = httpdate::fmt_http_date(SystemTime::UNIX_EPOCH);
    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(SequenceResponder::new(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", retry_after.as_str())
                .set_body_json(serde_json::json!({ "message": "slow down" })),
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })),
        ))
        .mount(&server)
        .await;

    let retry_policy = RetryPolicy::conservative()
        .max_retries(1)
        .base_delay(Duration::from_secs(1))
        .max_delay(Duration::from_secs(1))
        .jitter(Jitter::None);

    let client = Client::builder(server.uri())?
        .retry_policy(retry_policy)
        .build()?;

    let health = tokio::time::timeout(Duration::from_millis(500), client.health().get()).await??;
    assert_eq!(health.status, "ok");

    let requests = server
        .received_requests()
        .await
        .ok_or_else(|| std::io::Error::other("wiremock did not record requests"))?;
    assert_eq!(requests.len(), 2);
    Ok(())
}

#[tokio::test]
async fn post_is_not_retried_without_idempotency_key() -> TestResult<()> {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/session"))
        .respond_with(ResponseTemplate::new(503).set_body_json(serde_json::json!({
            "message": "unavailable"
        })))
        .mount(&server)
        .await;

    let retry_policy = RetryPolicy::conservative()
        .max_retries(1)
        .base_delay(Duration::from_secs(0))
        .max_delay(Duration::from_secs(0))
        .jitter(Jitter::None);
    let client = Client::builder(server.uri())?
        .retry_policy(retry_policy)
        .build()?;

    let request = metabase::types::session::CreateSessionRequest::new(
        "user@example.com",
        secrecy::SecretString::from("pw"),
    );

    let err = match client.session().create(&request).await {
        Ok(_) => {
            return Err(std::io::Error::other("expected error").into());
        }
        Err(err) => err,
    };

    let Error::Api(api) = err else {
        return Err(std::io::Error::other("unexpected error type").into());
    };
    assert_eq!(api.status(), StatusCode::SERVICE_UNAVAILABLE);

    let requests = server
        .received_requests()
        .await
        .ok_or_else(|| std::io::Error::other("wiremock did not record requests"))?;
    assert_eq!(requests.len(), 1);
    Ok(())
}

#[tokio::test]
async fn post_is_retried_with_idempotency_key() -> TestResult<()> {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/session"))
        .and(header("idempotency-key", "KEY"))
        .respond_with(SequenceResponder::new(
            ResponseTemplate::new(503).set_body_json(serde_json::json!({
                "message": "unavailable"
            })),
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "TOKEN"
            })),
        ))
        .mount(&server)
        .await;

    let retry_policy = RetryPolicy::conservative()
        .max_retries(1)
        .base_delay(Duration::from_secs(0))
        .max_delay(Duration::from_secs(0))
        .jitter(Jitter::None);
    let client = Client::builder(server.uri())?
        .retry_policy(retry_policy)
        .build()?;

    let request = metabase::types::session::CreateSessionRequest::new(
        "user@example.com",
        secrecy::SecretString::from("pw"),
    );
    let options = RequestOptions::new().idempotency_key(IdempotencyKey::new("KEY"));

    let response = client
        .session()
        .create_with_options(&request, options)
        .await?;
    assert_eq!(response.id, "TOKEN");

    let requests = server
        .received_requests()
        .await
        .ok_or_else(|| std::io::Error::other("wiremock did not record requests"))?;
    assert_eq!(requests.len(), 2);
    Ok(())
}
