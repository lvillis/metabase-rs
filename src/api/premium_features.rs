use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PremiumFeaturesService {
    client: Client,
}

#[cfg(feature = "async")]
impl PremiumFeaturesService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/premium-features/token/status
    pub async fn get_token_status(&self) -> Result<Value> {
        let segments = ["api", "premium-features", "token", "status"];
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
pub struct BlockingPremiumFeaturesService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPremiumFeaturesService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/premium-features/token/status
    pub fn get_token_status(&self) -> Result<Value> {
        let segments = ["api", "premium-features", "token", "status"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
