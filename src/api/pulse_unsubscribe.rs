use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PulseUnsubscribeService {
    client: Client,
}

#[cfg(feature = "async")]
impl PulseUnsubscribeService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/pulse/unsubscribe/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse", "unsubscribe"];
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

    /// POST /api/pulse/unsubscribe/undo
    pub async fn post_undo(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse", "unsubscribe", "undo"];
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
pub struct BlockingPulseUnsubscribeService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPulseUnsubscribeService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/pulse/unsubscribe/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse", "unsubscribe"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/pulse/unsubscribe/undo
    pub fn post_undo(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse", "unsubscribe", "undo"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
