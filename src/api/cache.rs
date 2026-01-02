use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct CacheService {
    client: Client,
}

#[cfg(feature = "async")]
impl CacheService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// DELETE /api/cache/
    pub async fn delete(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache"];
        self.client
            .request_json(
                Method::DELETE,
                &segments,
                Option::<&()>::None,
                body,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/cache/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache"];
        self.client
            .request_json(
                Method::GET,
                &segments,
                query,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// PUT /api/cache/
    pub async fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache"];
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

    /// POST /api/cache/invalidate
    pub async fn post_invalidate(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache", "invalidate"];
        self.client
            .request_json(
                Method::POST,
                &segments,
                query,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingCacheService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingCacheService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// DELETE /api/cache/
    pub fn delete(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/cache/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/cache/
    pub fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/cache/invalidate
    pub fn post_invalidate(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "cache", "invalidate"];
        self.client.request_json(
            Method::POST,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
