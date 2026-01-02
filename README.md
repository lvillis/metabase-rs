<h1 align="center"><code>metabase-rs</code></h1>

<p align="center">
Ergonomic Rust SDK for Metabase's HTTP API, with async and blocking clients.
</p>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/metabase.svg)](https://crates.io/crates/metabase)&nbsp;
[![Downloads](https://img.shields.io/crates/d/metabase.svg)](https://crates.io/crates/metabase)&nbsp;
[![Docs.rs](https://img.shields.io/docsrs/metabase)](https://docs.rs/metabase)&nbsp;
[![MSRV](https://img.shields.io/badge/MSRV-1.92-blue)](Cargo.toml)&nbsp;
[![CI](https://github.com/lvillis/metabase-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/lvillis/metabase-rs/actions/workflows/ci.yml)&nbsp;
[![Repo Size](https://img.shields.io/github/repo-size/lvillis/metabase-rs?color=328657)](https://github.com/lvillis/metabase-rs)&nbsp;
[![License](https://img.shields.io/crates/l/metabase.svg)](LICENSE)

</div>

---

## Features
- Async `Client` (default, `reqwest` + `rustls`)
- Blocking `BlockingClient` (`--features blocking`, `ureq` + `rustls`)
- Optional: `tracing`, `metrics`

## Install
```bash
cargo add metabase
# blocking-only:
cargo add metabase --no-default-features --features blocking
# with observability:
cargo add metabase --features tracing,metrics
```

## Usage (async)
```rust,no_run
use metabase::{Auth, Client};

# async fn demo() -> Result<(), metabase::Error> {
let client = Client::builder("https://metabase.example.com")?
    .auth(Auth::session("SESSION_TOKEN")) // or: Auth::api_key("API_KEY")
    .build()?;

let me = client.user().get_current().await?;
println!("{me:?}");
# Ok(())
# }
```

## Usage (blocking)
```rust,no_run
use metabase::{Auth, BlockingClient};

fn demo() -> Result<(), metabase::Error> {
    let client = BlockingClient::builder("https://metabase.example.com")?
        .auth(Auth::session("SESSION_TOKEN"))
        .build()?;

    let health = client.health().get()?;
    println!("{health:?}");
    Ok(())
}
```

## Errors
```rust,no_run
# use metabase::{Client, Error};
# async fn demo() -> Result<(), Error> {
# let client = Client::builder("https://metabase.example.com")?.build()?;
if let Err(err) = client.health().get().await {
    eprintln!("{err} status={:?} request_id={:?}", err.status(), err.request_id());
}
# Ok(())
# }
```
