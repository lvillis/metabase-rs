use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct SearchService {
    client: Client,
}

#[cfg(feature = "async")]
impl SearchService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/search/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "search"];
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

    /// POST /api/search/force-reindex
    pub async fn post_force_reindex(&self) -> Result<Value> {
        let segments = ["api", "search", "force-reindex"];
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

    /// POST /api/search/re-init
    pub async fn post_re_init(&self) -> Result<Value> {
        let segments = ["api", "search", "re-init"];
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

    /// GET /api/search/weights
    pub async fn get_weights(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "search", "weights"];
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

    /// PUT /api/search/weights
    pub async fn put_weights(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "search", "weights"];
        self.client
            .request_json(
                Method::PUT,
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
pub struct BlockingSearchService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingSearchService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/search/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "search"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/search/force-reindex
    pub fn post_force_reindex(&self) -> Result<Value> {
        let segments = ["api", "search", "force-reindex"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/search/re-init
    pub fn post_re_init(&self) -> Result<Value> {
        let segments = ["api", "search", "re-init"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/search/weights
    pub fn get_weights(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "search", "weights"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/search/weights
    pub fn put_weights(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "search", "weights"];
        self.client.request_json(
            Method::PUT,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
