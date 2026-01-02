#[cfg(feature = "blocking")]
use metabase::{Auth, BlockingClient};

#[cfg(feature = "blocking")]
fn main() -> Result<(), metabase::Error> {
    let base_url = match std::env::var("METABASE_BASE_URL") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Set METABASE_BASE_URL to run this example.");
            return Ok(());
        }
    };
    let session_token = match std::env::var("METABASE_SESSION") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Set METABASE_SESSION to run this example.");
            return Ok(());
        }
    };

    let client = BlockingClient::builder(base_url)?
        .auth(Auth::session(session_token))
        .build()?;

    let health = client.health().get()?;
    println!("{health:?}");
    Ok(())
}

#[cfg(not(feature = "blocking"))]
fn main() {
    eprintln!("This example requires the `blocking` feature.");
}
