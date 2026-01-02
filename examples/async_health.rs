#[cfg(feature = "async")]
use metabase::{Auth, Client};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), metabase::Error> {
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

    let client = Client::builder(base_url)?
        .auth(Auth::session(session_token))
        .build()?;

    let health = client.health().get().await?;
    println!("{health:?}");
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires the `async` feature.");
}
