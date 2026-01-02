use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct BugReportingService {
    client: Client,
}

#[cfg(feature = "async")]
impl BugReportingService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/bug-reporting/connection-pool-details
    pub async fn get_connection_pool_details(&self) -> Result<Value> {
        let segments = ["api", "bug-reporting", "connection-pool-details"];
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

    /// GET /api/bug-reporting/details
    pub async fn get_details(&self) -> Result<Value> {
        let segments = ["api", "bug-reporting", "details"];
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
pub struct BlockingBugReportingService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingBugReportingService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/bug-reporting/connection-pool-details
    pub fn get_connection_pool_details(&self) -> Result<Value> {
        let segments = ["api", "bug-reporting", "connection-pool-details"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/bug-reporting/details
    pub fn get_details(&self) -> Result<Value> {
        let segments = ["api", "bug-reporting", "details"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
