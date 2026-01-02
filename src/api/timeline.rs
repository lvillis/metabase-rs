use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct TimelineService {
    client: Client,
}

#[cfg(feature = "async")]
impl TimelineService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/timeline/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline"];
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

    /// POST /api/timeline/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline"];
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

    /// GET /api/timeline/collection/root
    pub async fn get_collection_root(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline", "collection", "root"];
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

    /// GET /api/timeline/collection/{id}
    pub async fn get_collection_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", "collection", id.as_str()];
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

    /// DELETE /api/timeline/{id}
    pub async fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", id.as_str()];
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

    /// GET /api/timeline/{id}
    pub async fn get_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", id.as_str()];
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

    /// PUT /api/timeline/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", id.as_str()];
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
pub struct BlockingTimelineService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingTimelineService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/timeline/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/timeline/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/timeline/collection/root
    pub fn get_collection_root(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline", "collection", "root"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/timeline/collection/{id}
    pub fn get_collection_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", "collection", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/timeline/{id}
    pub fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/timeline/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>, query: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/timeline/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
