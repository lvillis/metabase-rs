use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct UserService {
    client: Client,
}

#[cfg(feature = "async")]
impl UserService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/user/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "user"];
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

    /// POST /api/user/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "user"];
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

    /// GET /api/user/current
    pub async fn get_current(&self) -> Result<Value> {
        let segments = ["api", "user", "current"];
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

    /// GET /api/user/recipients
    pub async fn get_recipients(&self) -> Result<Value> {
        let segments = ["api", "user", "recipients"];
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

    /// DELETE /api/user/{id}
    pub async fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str()];
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

    /// GET /api/user/{id}
    pub async fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str()];
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

    /// PUT /api/user/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str()];
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

    /// PUT /api/user/{id}/modal/{modal}
    pub async fn put_by_id_modal_by_modal(
        &self,
        id: impl Into<PathParam>,
        modal: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let modal = modal.into();
        let segments = ["api", "user", id.as_str(), "modal", modal.as_str()];
        self.client
            .request_json(
                Method::PUT,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// PUT /api/user/{id}/password
    pub async fn put_by_id_password(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str(), "password"];
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

    /// PUT /api/user/{id}/reactivate
    pub async fn put_by_id_reactivate(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str(), "reactivate"];
        self.client
            .request_json(
                Method::PUT,
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
pub struct BlockingUserService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingUserService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/user/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "user"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/user/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "user"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/user/current
    pub fn get_current(&self) -> Result<Value> {
        let segments = ["api", "user", "current"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/user/recipients
    pub fn get_recipients(&self) -> Result<Value> {
        let segments = ["api", "user", "recipients"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/user/{id}
    pub fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/user/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/user/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// PUT /api/user/{id}/modal/{modal}
    pub fn put_by_id_modal_by_modal(
        &self,
        id: impl Into<PathParam>,
        modal: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let modal = modal.into();
        let segments = ["api", "user", id.as_str(), "modal", modal.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/user/{id}/password
    pub fn put_by_id_password(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str(), "password"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// PUT /api/user/{id}/reactivate
    pub fn put_by_id_reactivate(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "user", id.as_str(), "reactivate"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
