use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct CloudMigrationService {
    client: Client,
}

#[cfg(feature = "async")]
impl CloudMigrationService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/cloud-migration/
    pub async fn get(&self) -> Result<Value> {
        let segments = ["api", "cloud-migration"];
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

    /// POST /api/cloud-migration/
    pub async fn post(&self) -> Result<Value> {
        let segments = ["api", "cloud-migration"];
        self.client
            .request_json(
                Method::POST,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// PUT /api/cloud-migration/cancel
    pub async fn put_cancel(&self) -> Result<Value> {
        let segments = ["api", "cloud-migration", "cancel"];
        self.client
            .request_json(
                Method::PUT,
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
pub struct BlockingCloudMigrationService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingCloudMigrationService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/cloud-migration/
    pub fn get(&self) -> Result<Value> {
        let segments = ["api", "cloud-migration"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/cloud-migration/
    pub fn post(&self) -> Result<Value> {
        let segments = ["api", "cloud-migration"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/cloud-migration/cancel
    pub fn put_cancel(&self) -> Result<Value> {
        let segments = ["api", "cloud-migration", "cancel"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
