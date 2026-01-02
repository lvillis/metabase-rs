use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct LoginHistoryService {
    client: Client,
}

#[cfg(feature = "async")]
impl LoginHistoryService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/login-history/current
    pub async fn get_current(&self) -> Result<Value> {
        let segments = ["api", "login-history", "current"];
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
pub struct BlockingLoginHistoryService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingLoginHistoryService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/login-history/current
    pub fn get_current(&self) -> Result<Value> {
        let segments = ["api", "login-history", "current"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
