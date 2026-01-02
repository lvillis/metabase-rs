use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct UtilService {
    client: Client,
}

#[cfg(feature = "async")]
impl UtilService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/util/random_token
    pub async fn get_random_token(&self) -> Result<Value> {
        let segments = ["api", "util", "random_token"];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingUtilService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingUtilService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/util/random_token
    pub fn get_random_token(&self) -> Result<Value> {
        let segments = ["api", "util", "random_token"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
