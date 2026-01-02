#![cfg(feature = "blocking")]

use std::{
    collections::{HashMap, VecDeque},
    error::Error as StdError,
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use metabase::{
    BlockingClient, Error, IdempotencyKey, Jitter, RequestOptions, RetryPolicy,
    types::session::CreateSessionRequest,
};
use secrecy::SecretString;

type TestResult<T> = std::result::Result<T, Box<dyn StdError + Send + Sync>>;

#[derive(Clone, Debug)]
struct RecordedRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
}

#[derive(Clone, Debug)]
struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Response {
    fn json(status: u16, body: &str) -> Self {
        Self {
            status,
            headers: vec![("Content-Type".to_owned(), "application/json".to_owned())],
            body: body.as_bytes().to_vec(),
        }
    }
}

struct TestServer {
    addr: SocketAddr,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl TestServer {
    fn new(responses: Vec<Response>) -> TestResult<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;

        let requests = Arc::new(Mutex::new(Vec::new()));
        let responses = Arc::new(Mutex::new(VecDeque::from(responses)));
        let running = Arc::new(AtomicBool::new(true));

        let requests_thread = Arc::clone(&requests);
        let responses_thread = Arc::clone(&responses);
        let running_thread = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while running_thread.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        if !running_thread.load(Ordering::SeqCst) {
                            break;
                        }
                        let _ = handle_connection(stream, &requests_thread, &responses_thread);
                    }
                    Err(err) if err.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(_) => {}
                }
            }
        });

        Ok(Self {
            addr,
            requests,
            running,
            handle: Some(handle),
        })
    }

    fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn requests(&self) -> Vec<RecordedRequest> {
        self.requests
            .lock()
            .map(|requests| requests.clone())
            .unwrap_or_default()
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        let _ = TcpStream::connect(self.addr);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn handle_connection(
    stream: TcpStream,
    requests: &Arc<Mutex<Vec<RecordedRequest>>>,
    responses: &Arc<Mutex<VecDeque<Response>>>,
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;

    let mut reader = BufReader::new(stream);

    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        return Ok(());
    }
    if request_line == "\r\n" {
        return Ok(());
    }

    let mut parts = request_line.split_whitespace();
    let Some(method) = parts.next() else {
        return Ok(());
    };
    let Some(path) = parts.next() else {
        return Ok(());
    };

    let mut headers = HashMap::new();
    let mut content_length = 0usize;
    let mut is_chunked = false;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        if line == "\r\n" {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            let name = name.trim().to_ascii_lowercase();
            let value = value.trim().trim_end_matches('\r').to_owned();
            if name == "content-length"
                && let Ok(parsed) = value.parse::<usize>()
            {
                content_length = parsed;
            }
            if name == "transfer-encoding" && value.eq_ignore_ascii_case("chunked") {
                is_chunked = true;
            }
            headers.insert(name, value);
        }
    }

    if let Ok(mut guard) = requests.lock() {
        guard.push(RecordedRequest {
            method: method.to_owned(),
            path: path.to_owned(),
            headers,
        });
    }

    if content_length > 0 {
        let mut body = vec![0u8; content_length];
        reader.read_exact(&mut body)?;
    } else if is_chunked {
        read_chunked_body(&mut reader)?;
    }

    let response = responses
        .lock()
        .ok()
        .and_then(|mut q| q.pop_front())
        .unwrap_or_else(|| Response::json(500, r#"{"message":"no response queued"}"#));

    let mut stream = reader.into_inner();
    let reason = reason_phrase(response.status);
    let mut raw = Vec::new();
    let _ = write!(
        raw,
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nConnection: close\r\n",
        response.status,
        reason,
        response.body.len()
    );
    for (name, value) in &response.headers {
        let _ = write!(raw, "{name}: {value}\r\n");
    }
    raw.extend_from_slice(b"\r\n");
    raw.extend_from_slice(&response.body);

    stream.write_all(&raw)?;
    stream.flush()?;
    Ok(())
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}

fn read_chunked_body(reader: &mut BufReader<TcpStream>) -> std::io::Result<()> {
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Ok(());
        }
        let size = line.trim().trim_end_matches('\r');
        if size.is_empty() {
            continue;
        }
        let Ok(size) = usize::from_str_radix(size, 16) else {
            return Ok(());
        };

        if size == 0 {
            loop {
                let mut trailer = String::new();
                if reader.read_line(&mut trailer)? == 0 {
                    return Ok(());
                }
                if trailer == "\r\n" {
                    break;
                }
            }
            return Ok(());
        }

        let mut chunk = vec![0u8; size];
        reader.read_exact(&mut chunk)?;

        let mut crlf = [0u8; 2];
        reader.read_exact(&mut crlf)?;
    }
}

#[test]
fn blocking_health_get_ok() -> TestResult<()> {
    let server = TestServer::new(vec![Response::json(200, r#"{"status":"ok"}"#)])?;

    let client = BlockingClient::builder(server.base_url())?.build()?;
    let health = client.health().get()?;

    assert_eq!(health.status, "ok");
    assert_eq!(server.requests().len(), 1);
    Ok(())
}

#[test]
fn blocking_get_retries_on_503() -> TestResult<()> {
    let mut first = Response::json(503, r#"{"message":"unavailable"}"#);
    first
        .headers
        .push(("Retry-After".to_owned(), "0".to_owned()));

    let server = TestServer::new(vec![first, Response::json(200, r#"{"status":"ok"}"#)])?;

    let client = BlockingClient::builder(server.base_url())?.build()?;

    let result = client.health().get();
    let requests = server.requests();
    assert_eq!(requests.len(), 2);

    let health = result?;
    assert_eq!(health.status, "ok");
    Ok(())
}

#[test]
fn blocking_post_is_not_retried_without_idempotency_key() -> TestResult<()> {
    let server = TestServer::new(vec![Response::json(503, r#"{"message":"unavailable"}"#)])?;

    let retry_policy = RetryPolicy::conservative()
        .max_retries(1)
        .base_delay(Duration::from_secs(0))
        .max_delay(Duration::from_secs(0))
        .jitter(Jitter::None);

    let client = BlockingClient::builder(server.base_url())?
        .retry_policy(retry_policy)
        .build()?;

    let request = CreateSessionRequest::new("user@example.com", SecretString::from("pw"));

    let err = match client.session().create(&request) {
        Ok(_) => {
            return Err(std::io::Error::other("expected error").into());
        }
        Err(err) => err,
    };

    let Error::Api(api) = err else {
        eprintln!("unexpected error: {err:?}");
        eprintln!("server requests: {:#?}", server.requests());
        return Err(std::io::Error::other("unexpected error type").into());
    };
    assert_eq!(api.status(), http::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(server.requests().len(), 1);
    Ok(())
}

#[test]
fn blocking_post_is_retried_with_idempotency_key() -> TestResult<()> {
    let mut first = Response::json(503, r#"{"message":"unavailable"}"#);
    first
        .headers
        .push(("Retry-After".to_owned(), "0".to_owned()));

    let server = TestServer::new(vec![first, Response::json(200, r#"{"id":"TOKEN"}"#)])?;

    let client = BlockingClient::builder(server.base_url())?.build()?;

    let request = CreateSessionRequest::new("user@example.com", SecretString::from("pw"));
    let options = RequestOptions::new().idempotency_key(IdempotencyKey::new("KEY"));

    let result = client.session().create_with_options(&request, options);

    let requests = server.requests();
    assert_eq!(requests.len(), 2);
    for req in requests {
        assert_eq!(req.method, "POST");
        assert_eq!(req.path, "/api/session");
        assert_eq!(
            req.headers.get("idempotency-key").map(String::as_str),
            Some("KEY")
        );
    }

    let response = result?;
    assert_eq!(response.id, "TOKEN");
    Ok(())
}
