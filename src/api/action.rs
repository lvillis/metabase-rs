use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct ActionService {
    client: Client,
}

#[cfg(feature = "async")]
impl ActionService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/action/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "action"];
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

    /// POST /api/action/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "action"];
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

    /// GET /api/action/public
    pub async fn get_public(&self) -> Result<Value> {
        let segments = ["api", "action", "public"];
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

    /// DELETE /api/action/{action-id}
    pub async fn delete_by_action_id(&self, action_id: impl Into<PathParam>) -> Result<Value> {
        let action_id = action_id.into();
        let segments = ["api", "action", action_id.as_str()];
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

    /// GET /api/action/{action-id}
    pub async fn get_by_action_id(&self, action_id: impl Into<PathParam>) -> Result<Value> {
        let action_id = action_id.into();
        let segments = ["api", "action", action_id.as_str()];
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

    /// GET /api/action/{action-id}/execute
    pub async fn get_by_action_id_execute(
        &self,
        action_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let action_id = action_id.into();
        let segments = ["api", "action", action_id.as_str(), "execute"];
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

    /// PUT /api/action/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str()];
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

    /// POST /api/action/{id}/execute
    pub async fn post_by_id_execute(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str(), "execute"];
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

    /// DELETE /api/action/{id}/public_link
    pub async fn delete_by_id_public_link(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str(), "public_link"];
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

    /// POST /api/action/{id}/public_link
    pub async fn post_by_id_public_link(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str(), "public_link"];
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
pub struct BlockingActionService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingActionService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/action/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "action"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/action/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "action"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/action/public
    pub fn get_public(&self) -> Result<Value> {
        let segments = ["api", "action", "public"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/action/{action-id}
    pub fn delete_by_action_id(&self, action_id: impl Into<PathParam>) -> Result<Value> {
        let action_id = action_id.into();
        let segments = ["api", "action", action_id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/action/{action-id}
    pub fn get_by_action_id(&self, action_id: impl Into<PathParam>) -> Result<Value> {
        let action_id = action_id.into();
        let segments = ["api", "action", action_id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/action/{action-id}/execute
    pub fn get_by_action_id_execute(
        &self,
        action_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let action_id = action_id.into();
        let segments = ["api", "action", action_id.as_str(), "execute"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/action/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/action/{id}/execute
    pub fn post_by_id_execute(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str(), "execute"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/action/{id}/public_link
    pub fn delete_by_id_public_link(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str(), "public_link"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/action/{id}/public_link
    pub fn post_by_id_public_link(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "action", id.as_str(), "public_link"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
