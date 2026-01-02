#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), metabase::Error> {
    run_async().await
}

#[cfg(all(not(feature = "async"), feature = "blocking"))]
fn main() -> Result<(), metabase::Error> {
    run_blocking()
}

#[cfg(all(not(feature = "async"), not(feature = "blocking")))]
fn main() {
    eprintln!("Enable at least one of: `async`, `blocking`.");
}

#[cfg(feature = "async")]
async fn run_async() -> Result<(), metabase::Error> {
    use metabase::{Auth, Client, Error};

    let base_url = match std::env::var("METABASE_BASE_URL") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Set METABASE_BASE_URL to run this example.");
            return Ok(());
        }
    };
    let session_token = std::env::var("METABASE_SESSION").ok();

    let mut builder = Client::builder(base_url)?;
    if let Some(token) = session_token {
        builder = builder.auth(Auth::session(token));
    }
    let client = builder.build()?;

    match client.health().get().await {
        Ok(health) => println!("{health:?}"),
        Err(err) => {
            eprintln!("{err}");
            if let Some(status) = err.status() {
                eprintln!("status={status}");
            }
            if let Some(request_id) = err.request_id() {
                eprintln!("request_id={request_id}");
            }
            if let Error::RateLimited(rate_limited) = &err {
                eprintln!("retry_after={:?}", rate_limited.retry_after());
            }
        }
    }

    Ok(())
}

#[cfg(all(not(feature = "async"), feature = "blocking"))]
fn run_blocking() -> Result<(), metabase::Error> {
    use metabase::{Auth, BlockingClient, Error};

    let base_url = match std::env::var("METABASE_BASE_URL") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Set METABASE_BASE_URL to run this example.");
            return Ok(());
        }
    };
    let session_token = std::env::var("METABASE_SESSION").ok();

    let mut builder = BlockingClient::builder(base_url)?;
    if let Some(token) = session_token {
        builder = builder.auth(Auth::session(token));
    }
    let client = builder.build()?;

    match client.health().get() {
        Ok(health) => println!("{health:?}"),
        Err(err) => {
            eprintln!("{err}");
            if let Some(status) = err.status() {
                eprintln!("status={status}");
            }
            if let Some(request_id) = err.request_id() {
                eprintln!("request_id={request_id}");
            }
            if let Error::RateLimited(rate_limited) = &err {
                eprintln!("retry_after={:?}", rate_limited.retry_after());
            }
        }
    }

    Ok(())
}
