use crate::{Result, types::health::HealthResponse};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct HealthService {
    client: Client,
}

#[cfg(feature = "async")]
impl HealthService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<HealthResponse> {
        self.client
            .get_json(&["api", "health"], Option::<&()>::None)
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingHealthService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingHealthService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn get(&self) -> Result<HealthResponse> {
        self.client
            .get_json(&["api", "health"], Option::<&()>::None)
    }
}
