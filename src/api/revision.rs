use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct RevisionService {
    client: Client,
}

#[cfg(feature = "async")]
impl RevisionService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/revision/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "revision"];
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

    /// POST /api/revision/revert
    pub async fn post_revert(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "revision", "revert"];
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

    /// GET /api/revision/{entity}/{id}
    pub async fn get_by_entity_by_id(
        &self,
        entity: impl Into<PathParam>,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let entity = entity.into();
        let id = id.into();
        let segments = ["api", "revision", entity.as_str(), id.as_str()];
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
pub struct BlockingRevisionService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingRevisionService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/revision/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "revision"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/revision/revert
    pub fn post_revert(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "revision", "revert"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/revision/{entity}/{id}
    pub fn get_by_entity_by_id(
        &self,
        entity: impl Into<PathParam>,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let entity = entity.into();
        let id = id.into();
        let segments = ["api", "revision", entity.as_str(), id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
