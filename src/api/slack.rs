use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct SlackService {
    client: Client,
}

#[cfg(feature = "async")]
impl SlackService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/slack/bug-report
    pub async fn post_bug_report(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "slack", "bug-report"];
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

    /// GET /api/slack/manifest
    pub async fn get_manifest(&self) -> Result<Value> {
        let segments = ["api", "slack", "manifest"];
        self.client
            .request_json(
                Method::GET,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// PUT /api/slack/settings
    pub async fn put_settings(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "slack", "settings"];
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
pub struct BlockingSlackService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingSlackService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/slack/bug-report
    pub fn post_bug_report(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "slack", "bug-report"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/slack/manifest
    pub fn get_manifest(&self) -> Result<Value> {
        let segments = ["api", "slack", "manifest"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/slack/settings
    pub fn put_settings(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "slack", "settings"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
