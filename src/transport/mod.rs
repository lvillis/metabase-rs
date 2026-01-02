#[cfg(feature = "async")]
mod async_transport;
#[cfg(feature = "blocking")]
mod blocking_transport;
mod retry;

#[cfg(feature = "async")]
pub(crate) use async_transport::AsyncTransport;
#[cfg(feature = "blocking")]
pub(crate) use blocking_transport::BlockingTransport;
