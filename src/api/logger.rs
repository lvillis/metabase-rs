use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct LoggerService {
    client: Client,
}

#[cfg(feature = "async")]
impl LoggerService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// DELETE /api/logger/adjustment
    pub async fn delete_adjustment(&self) -> Result<Value> {
        let segments = ["api", "logger", "adjustment"];
        self.client
            .request_json(
                Method::DELETE,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// POST /api/logger/adjustment
    pub async fn post_adjustment(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "logger", "adjustment"];
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

    /// GET /api/logger/logs
    pub async fn get_logs(&self) -> Result<Value> {
        let segments = ["api", "logger", "logs"];
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

    /// GET /api/logger/presets
    pub async fn get_presets(&self) -> Result<Value> {
        let segments = ["api", "logger", "presets"];
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
pub struct BlockingLoggerService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingLoggerService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// DELETE /api/logger/adjustment
    pub fn delete_adjustment(&self) -> Result<Value> {
        let segments = ["api", "logger", "adjustment"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/logger/adjustment
    pub fn post_adjustment(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "logger", "adjustment"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/logger/logs
    pub fn get_logs(&self) -> Result<Value> {
        let segments = ["api", "logger", "logs"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/logger/presets
    pub fn get_presets(&self) -> Result<Value> {
        let segments = ["api", "logger", "presets"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
