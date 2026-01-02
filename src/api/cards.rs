use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct CardsService {
    client: Client,
}

#[cfg(feature = "async")]
impl CardsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/cards/dashboards
    pub async fn post_dashboards(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cards", "dashboards"];
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

    /// POST /api/cards/move
    pub async fn post_move(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cards", "move"];
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
pub struct BlockingCardsService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingCardsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/cards/dashboards
    pub fn post_dashboards(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cards", "dashboards"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/cards/move
    pub fn post_move(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cards", "move"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
