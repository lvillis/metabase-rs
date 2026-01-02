use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct CollectionService {
    client: Client,
}

#[cfg(feature = "async")]
impl CollectionService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/collection/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection"];
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

    /// POST /api/collection/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection"];
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

    /// GET /api/collection/graph
    pub async fn get_graph(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "graph"];
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

    /// PUT /api/collection/graph
    pub async fn put_graph(&self, query: Option<&Value>, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "graph"];
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

    /// GET /api/collection/root
    pub async fn get_root(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "root"];
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

    /// GET /api/collection/root/dashboard-question-candidates
    pub async fn get_root_dashboard_question_candidates(&self) -> Result<Value> {
        let segments = ["api", "collection", "root", "dashboard-question-candidates"];
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

    /// GET /api/collection/root/items
    pub async fn get_root_items(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "root", "items"];
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

    /// POST /api/collection/root/move-dashboard-question-candidates
    pub async fn post_root_move_dashboard_question_candidates(
        &self,
        body: Option<&Value>,
    ) -> Result<Value> {
        let segments = [
            "api",
            "collection",
            "root",
            "move-dashboard-question-candidates",
        ];
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

    /// GET /api/collection/trash
    pub async fn get_trash(&self) -> Result<Value> {
        let segments = ["api", "collection", "trash"];
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

    /// GET /api/collection/tree
    pub async fn get_tree(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "tree"];
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

    /// DELETE /api/collection/{id}
    pub async fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str()];
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

    /// GET /api/collection/{id}
    pub async fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str()];
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

    /// PUT /api/collection/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str()];
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

    /// GET /api/collection/{id}/dashboard-question-candidates
    pub async fn get_by_id_dashboard_question_candidates(
        &self,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "collection",
            id.as_str(),
            "dashboard-question-candidates",
        ];
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

    /// GET /api/collection/{id}/items
    pub async fn get_by_id_items(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str(), "items"];
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

    /// POST /api/collection/{id}/move-dashboard-question-candidates
    pub async fn post_by_id_move_dashboard_question_candidates(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "collection",
            id.as_str(),
            "move-dashboard-question-candidates",
        ];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingCollectionService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingCollectionService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/collection/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/collection/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/graph
    pub fn get_graph(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "graph"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/collection/graph
    pub fn put_graph(&self, query: Option<&Value>, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "graph"];
        self.client.request_json(
            Method::PUT,
            &segments,
            query,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/root
    pub fn get_root(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "root"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/root/dashboard-question-candidates
    pub fn get_root_dashboard_question_candidates(&self) -> Result<Value> {
        let segments = ["api", "collection", "root", "dashboard-question-candidates"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/root/items
    pub fn get_root_items(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "root", "items"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/collection/root/move-dashboard-question-candidates
    pub fn post_root_move_dashboard_question_candidates(
        &self,
        body: Option<&Value>,
    ) -> Result<Value> {
        let segments = [
            "api",
            "collection",
            "root",
            "move-dashboard-question-candidates",
        ];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/trash
    pub fn get_trash(&self) -> Result<Value> {
        let segments = ["api", "collection", "trash"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/tree
    pub fn get_tree(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "collection", "tree"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/collection/{id}
    pub fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/collection/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/{id}/dashboard-question-candidates
    pub fn get_by_id_dashboard_question_candidates(
        &self,
        id: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "collection",
            id.as_str(),
            "dashboard-question-candidates",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/collection/{id}/items
    pub fn get_by_id_items(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "collection", id.as_str(), "items"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/collection/{id}/move-dashboard-question-candidates
    pub fn post_by_id_move_dashboard_question_candidates(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "collection",
            id.as_str(),
            "move-dashboard-question-candidates",
        ];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
