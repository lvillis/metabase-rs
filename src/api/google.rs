use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct GoogleService {
    client: Client,
}

#[cfg(feature = "async")]
impl GoogleService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// PUT /api/google/settings
    pub async fn put_settings(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "google", "settings"];
        self.client
            .request_json(
                Method::PUT,
                &segments,
                Option::<&()>::None,
                body,
                RequestOptions::default(),
            )
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingGoogleService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingGoogleService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// PUT /api/google/settings
    pub fn put_settings(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "google", "settings"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
