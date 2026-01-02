use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct BookmarkService {
    client: Client,
}

#[cfg(feature = "async")]
impl BookmarkService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/bookmark/
    pub async fn get(&self) -> Result<Value> {
        let segments = ["api", "bookmark"];
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

    /// PUT /api/bookmark/ordering
    pub async fn put_ordering(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "bookmark", "ordering"];
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

    /// DELETE /api/bookmark/{model}/{id}
    pub async fn delete_by_model_by_id(
        &self,
        model: impl Into<PathParam>,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let model = model.into();
        let id = id.into();
        let segments = ["api", "bookmark", model.as_str(), id.as_str()];
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

    /// POST /api/bookmark/{model}/{id}
    pub async fn post_by_model_by_id(
        &self,
        model: impl Into<PathParam>,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let model = model.into();
        let id = id.into();
        let segments = ["api", "bookmark", model.as_str(), id.as_str()];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingBookmarkService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingBookmarkService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/bookmark/
    pub fn get(&self) -> Result<Value> {
        let segments = ["api", "bookmark"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/bookmark/ordering
    pub fn put_ordering(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "bookmark", "ordering"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/bookmark/{model}/{id}
    pub fn delete_by_model_by_id(
        &self,
        model: impl Into<PathParam>,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let model = model.into();
        let id = id.into();
        let segments = ["api", "bookmark", model.as_str(), id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/bookmark/{model}/{id}
    pub fn post_by_model_by_id(
        &self,
        model: impl Into<PathParam>,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let model = model.into();
        let id = id.into();
        let segments = ["api", "bookmark", model.as_str(), id.as_str()];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
