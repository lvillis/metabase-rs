mod config;
mod options;
mod retry;

#[cfg(feature = "async")]
mod async_client;
#[cfg(feature = "blocking")]
mod blocking_client;
#[cfg(feature = "async")]
mod services_async;
#[cfg(feature = "blocking")]
mod services_blocking;

pub(crate) use config::ClientConfig;
pub use options::{IdempotencyKey, RequestOptions};
pub use retry::{Jitter, RetryPolicy};

#[cfg(feature = "async")]
pub use async_client::{Client, ClientBuilder};
#[cfg(feature = "blocking")]
pub use blocking_client::{BlockingClient, BlockingClientBuilder};
