use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PermissionsService {
    client: Client,
}

#[cfg(feature = "async")]
impl PermissionsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/permissions/graph
    pub async fn get_graph(&self) -> Result<Value> {
        let segments = ["api", "permissions", "graph"];
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

    /// PUT /api/permissions/graph
    pub async fn put_graph(&self, query: Option<&Value>, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "permissions", "graph"];
        self.client
            .request_json(
                Method::PUT,
                &segments,
                query,
                body,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/permissions/graph/db/{db-id}
    pub async fn get_graph_db_by_db_id(&self, db_id: impl Into<PathParam>) -> Result<Value> {
        let db_id = db_id.into();
        let segments = ["api", "permissions", "graph", "db", db_id.as_str()];
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

    /// GET /api/permissions/graph/group/{group-id}
    pub async fn get_graph_group_by_group_id(
        &self,
        group_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let group_id = group_id.into();
        let segments = ["api", "permissions", "graph", "group", group_id.as_str()];
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

    /// GET /api/permissions/group
    pub async fn get_group(&self) -> Result<Value> {
        let segments = ["api", "permissions", "group"];
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

    /// POST /api/permissions/group
    pub async fn post_group(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "permissions", "group"];
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

    /// DELETE /api/permissions/group/{group-id}
    pub async fn delete_group_by_group_id(&self, group_id: impl Into<PathParam>) -> Result<Value> {
        let group_id = group_id.into();
        let segments = ["api", "permissions", "group", group_id.as_str()];
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

    /// PUT /api/permissions/group/{group-id}
    pub async fn put_group_by_group_id(
        &self,
        group_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let group_id = group_id.into();
        let segments = ["api", "permissions", "group", group_id.as_str()];
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

    /// GET /api/permissions/group/{id}
    pub async fn get_group_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "permissions", "group", id.as_str()];
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

    /// GET /api/permissions/membership
    pub async fn get_membership(&self) -> Result<Value> {
        let segments = ["api", "permissions", "membership"];
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

    /// POST /api/permissions/membership
    pub async fn post_membership(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "permissions", "membership"];
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

    /// PUT /api/permissions/membership/{group-id}/clear
    pub async fn put_membership_by_group_id_clear(
        &self,
        group_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let group_id = group_id.into();
        let segments = [
            "api",
            "permissions",
            "membership",
            group_id.as_str(),
            "clear",
        ];
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

    /// DELETE /api/permissions/membership/{id}
    pub async fn delete_membership_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "permissions", "membership", id.as_str()];
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

    /// PUT /api/permissions/membership/{id}
    pub async fn put_membership_by_id(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "permissions", "membership", id.as_str()];
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
pub struct BlockingPermissionsService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPermissionsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/permissions/graph
    pub fn get_graph(&self) -> Result<Value> {
        let segments = ["api", "permissions", "graph"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/permissions/graph
    pub fn put_graph(&self, query: Option<&Value>, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "permissions", "graph"];
        self.client.request_json(
            Method::PUT,
            &segments,
            query,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/permissions/graph/db/{db-id}
    pub fn get_graph_db_by_db_id(&self, db_id: impl Into<PathParam>) -> Result<Value> {
        let db_id = db_id.into();
        let segments = ["api", "permissions", "graph", "db", db_id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/permissions/graph/group/{group-id}
    pub fn get_graph_group_by_group_id(&self, group_id: impl Into<PathParam>) -> Result<Value> {
        let group_id = group_id.into();
        let segments = ["api", "permissions", "graph", "group", group_id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/permissions/group
    pub fn get_group(&self) -> Result<Value> {
        let segments = ["api", "permissions", "group"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/permissions/group
    pub fn post_group(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "permissions", "group"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/permissions/group/{group-id}
    pub fn delete_group_by_group_id(&self, group_id: impl Into<PathParam>) -> Result<Value> {
        let group_id = group_id.into();
        let segments = ["api", "permissions", "group", group_id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/permissions/group/{group-id}
    pub fn put_group_by_group_id(
        &self,
        group_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let group_id = group_id.into();
        let segments = ["api", "permissions", "group", group_id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/permissions/group/{id}
    pub fn get_group_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "permissions", "group", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/permissions/membership
    pub fn get_membership(&self) -> Result<Value> {
        let segments = ["api", "permissions", "membership"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/permissions/membership
    pub fn post_membership(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "permissions", "membership"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// PUT /api/permissions/membership/{group-id}/clear
    pub fn put_membership_by_group_id_clear(
        &self,
        group_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let group_id = group_id.into();
        let segments = [
            "api",
            "permissions",
            "membership",
            group_id.as_str(),
            "clear",
        ];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/permissions/membership/{id}
    pub fn delete_membership_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "permissions", "membership", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/permissions/membership/{id}
    pub fn put_membership_by_id(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "permissions", "membership", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
