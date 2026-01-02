use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct EidTranslationService {
    client: Client,
}

#[cfg(feature = "async")]
impl EidTranslationService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/eid-translation/translate
    pub async fn post_translate(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "eid-translation", "translate"];
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
pub struct BlockingEidTranslationService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingEidTranslationService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/eid-translation/translate
    pub fn post_translate(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "eid-translation", "translate"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
