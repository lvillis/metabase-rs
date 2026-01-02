use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct SettingService {
    client: Client,
}

#[cfg(feature = "async")]
impl SettingService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/setting/
    pub async fn get(&self) -> Result<Value> {
        let segments = ["api", "setting"];
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

    /// PUT /api/setting/
    pub async fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "setting"];
        self.client
            .request_json(
                Method::PUT,
                &segments,
                Option::<&()>::None,
                body,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/setting/{key}
    pub async fn get_by_key(&self, key: impl Into<PathParam>) -> Result<Value> {
        let key = key.into();
        let segments = ["api", "setting", key.as_str()];
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

    /// PUT /api/setting/{key}
    pub async fn put_by_key(
        &self,
        key: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let key = key.into();
        let segments = ["api", "setting", key.as_str()];
        self.client
            .request_json(
                Method::PUT,
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
pub struct BlockingSettingService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingSettingService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/setting/
    pub fn get(&self) -> Result<Value> {
        let segments = ["api", "setting"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/setting/
    pub fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "setting"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/setting/{key}
    pub fn get_by_key(&self, key: impl Into<PathParam>) -> Result<Value> {
        let key = key.into();
        let segments = ["api", "setting", key.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/setting/{key}
    pub fn put_by_key(&self, key: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let key = key.into();
        let segments = ["api", "setting", key.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
