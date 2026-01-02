use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct SetupService {
    client: Client,
}

#[cfg(feature = "async")]
impl SetupService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/setup/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "setup"];
        self.client
            .request_json(
                Method::POST,
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
pub struct BlockingSetupService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingSetupService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/setup/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "setup"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
