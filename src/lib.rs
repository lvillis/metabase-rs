//! Rust SDK for the Metabase HTTP API.
//!
//! ## Quick start (async)
//! ```no_run
//! # #[cfg(feature = "async")]
//! # async fn demo() -> Result<(), metabase::Error> {
//! use metabase::{Auth, Client};
//!
//! let client = Client::builder("https://metabase.example.com")?
//!     .auth(Auth::session("SESSION_TOKEN"))
//!     .build()?;
//!
//! let health = client.health().get().await?;
//! println!("{health:?}");
//! # Ok(())
//! # }
//! ```
//!
//! ## Quick start (blocking)
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! # fn demo() -> Result<(), metabase::Error> {
//! use metabase::{Auth, BlockingClient};
//!
//! let client = BlockingClient::builder("https://metabase.example.com")?
//!     .auth(Auth::session("SESSION_TOKEN"))
//!     .build()?;
//!
//! let health = client.health().get()?;
//! println!("{health:?}");
//! # Ok(())
//! # }
//! ```

#[cfg(all(not(feature = "async"), not(feature = "blocking")))]
compile_error!("Enable at least one of: `async`, `blocking`.");

mod transport;
mod util;

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod types;

pub use auth::Auth;
#[cfg(feature = "blocking")]
pub use client::BlockingClient;
#[cfg(feature = "async")]
pub use client::Client;
pub use client::{IdempotencyKey, Jitter, RequestOptions, RetryPolicy};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
