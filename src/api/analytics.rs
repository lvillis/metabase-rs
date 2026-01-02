use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct AnalyticsService {
    client: Client,
}

#[cfg(feature = "async")]
impl AnalyticsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/analytics/anonymous-stats
    pub async fn get_anonymous_stats(&self) -> Result<Value> {
        let segments = ["api", "analytics", "anonymous-stats"];
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
pub struct BlockingAnalyticsService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingAnalyticsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/analytics/anonymous-stats
    pub fn get_anonymous_stats(&self) -> Result<Value> {
        let segments = ["api", "analytics", "anonymous-stats"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
