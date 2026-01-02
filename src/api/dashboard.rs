use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct DashboardService {
    client: Client,
}

#[cfg(feature = "async")]
impl DashboardService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/dashboard/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dashboard"];
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

    /// POST /api/dashboard/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dashboard"];
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

    /// GET /api/dashboard/embeddable
    pub async fn get_embeddable(&self) -> Result<Value> {
        let segments = ["api", "dashboard", "embeddable"];
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

    /// GET /api/dashboard/params/valid-filter-fields
    pub async fn get_params_valid_filter_fields(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dashboard", "params", "valid-filter-fields"];
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

    /// POST /api/dashboard/pivot/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/query
    pub async fn post_pivot_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_query(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "dashboard",
            "pivot",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            "query",
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

    /// GET /api/dashboard/public
    pub async fn get_public(&self) -> Result<Value> {
        let segments = ["api", "dashboard", "public"];
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

    /// POST /api/dashboard/save
    pub async fn post_save(&self) -> Result<Value> {
        let segments = ["api", "dashboard", "save"];
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

    /// POST /api/dashboard/save/collection/{parent-collection-id}
    pub async fn post_save_collection_by_parent_collection_id(
        &self,
        parent_collection_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let parent_collection_id = parent_collection_id.into();
        let segments = [
            "api",
            "dashboard",
            "save",
            "collection",
            parent_collection_id.as_str(),
        ];
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

    /// POST /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/query
    pub async fn post_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_query(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            "query",
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

    /// POST /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/query/{export-format}
    pub async fn post_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_query_by_export_format(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let export_format = export_format.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            "query",
            export_format.as_str(),
        ];
        self.client
            .request_bytes(
                Method::POST,
                &segments,
                Option::<&()>::None,
                body,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/execute
    pub async fn get_by_dashboard_id_dashcard_by_dashcard_id_execute(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "execute",
        ];
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

    /// POST /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/execute
    pub async fn post_by_dashboard_id_dashcard_by_dashcard_id_execute(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "execute",
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

    /// DELETE /api/dashboard/{dashboard-id}/public_link
    pub async fn delete_by_dashboard_id_public_link(
        &self,
        dashboard_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let segments = ["api", "dashboard", dashboard_id.as_str(), "public_link"];
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

    /// POST /api/dashboard/{dashboard-id}/public_link
    pub async fn post_by_dashboard_id_public_link(
        &self,
        dashboard_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let segments = ["api", "dashboard", dashboard_id.as_str(), "public_link"];
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

    /// POST /api/dashboard/{from-dashboard-id}/copy
    pub async fn post_by_from_dashboard_id_copy(
        &self,
        from_dashboard_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let from_dashboard_id = from_dashboard_id.into();
        let segments = ["api", "dashboard", from_dashboard_id.as_str(), "copy"];
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

    /// DELETE /api/dashboard/{id}
    pub async fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str()];
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

    /// GET /api/dashboard/{id}
    pub async fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str()];
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

    /// PUT /api/dashboard/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str()];
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

    /// PUT /api/dashboard/{id}/cards
    pub async fn put_by_id_cards(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "cards"];
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

    /// GET /api/dashboard/{id}/items
    pub async fn get_by_id_items(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "items"];
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

    /// GET /api/dashboard/{id}/params/{param-key}/remapping
    pub async fn get_by_id_params_by_param_key_remapping(
        &self,
        id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "dashboard",
            id.as_str(),
            "params",
            param_key.as_str(),
            "remapping",
        ];
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

    /// GET /api/dashboard/{id}/params/{param-key}/search/{query}
    pub async fn get_by_id_params_by_param_key_search_by_query(
        &self,
        id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "dashboard",
            id.as_str(),
            "params",
            param_key.as_str(),
            "search",
            query.as_str(),
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

    /// GET /api/dashboard/{id}/params/{param-key}/values
    pub async fn get_by_id_params_by_param_key_values(
        &self,
        id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "dashboard",
            id.as_str(),
            "params",
            param_key.as_str(),
            "values",
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

    /// GET /api/dashboard/{id}/query_metadata
    pub async fn get_by_id_query_metadata(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "query_metadata"];
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

    /// GET /api/dashboard/{id}/related
    pub async fn get_by_id_related(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "related"];
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
pub struct BlockingDashboardService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingDashboardService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/dashboard/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dashboard"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dashboard"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/embeddable
    pub fn get_embeddable(&self) -> Result<Value> {
        let segments = ["api", "dashboard", "embeddable"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/params/valid-filter-fields
    pub fn get_params_valid_filter_fields(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dashboard", "params", "valid-filter-fields"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/pivot/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/query
    pub fn post_pivot_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_query(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "dashboard",
            "pivot",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            "query",
        ];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/public
    pub fn get_public(&self) -> Result<Value> {
        let segments = ["api", "dashboard", "public"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/save
    pub fn post_save(&self) -> Result<Value> {
        let segments = ["api", "dashboard", "save"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/save/collection/{parent-collection-id}
    pub fn post_save_collection_by_parent_collection_id(
        &self,
        parent_collection_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let parent_collection_id = parent_collection_id.into();
        let segments = [
            "api",
            "dashboard",
            "save",
            "collection",
            parent_collection_id.as_str(),
        ];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/query
    pub fn post_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_query(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            "query",
        ];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/query/{export-format}
    pub fn post_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_query_by_export_format(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let export_format = export_format.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            "query",
            export_format.as_str(),
        ];
        self.client.request_bytes(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/execute
    pub fn get_by_dashboard_id_dashcard_by_dashcard_id_execute(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "execute",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/{dashboard-id}/dashcard/{dashcard-id}/execute
    pub fn post_by_dashboard_id_dashcard_by_dashcard_id_execute(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "dashboard",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "execute",
        ];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/dashboard/{dashboard-id}/public_link
    pub fn delete_by_dashboard_id_public_link(
        &self,
        dashboard_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let segments = ["api", "dashboard", dashboard_id.as_str(), "public_link"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/{dashboard-id}/public_link
    pub fn post_by_dashboard_id_public_link(
        &self,
        dashboard_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let dashboard_id = dashboard_id.into();
        let segments = ["api", "dashboard", dashboard_id.as_str(), "public_link"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/dashboard/{from-dashboard-id}/copy
    pub fn post_by_from_dashboard_id_copy(
        &self,
        from_dashboard_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let from_dashboard_id = from_dashboard_id.into();
        let segments = ["api", "dashboard", from_dashboard_id.as_str(), "copy"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/dashboard/{id}
    pub fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/dashboard/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// PUT /api/dashboard/{id}/cards
    pub fn put_by_id_cards(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "cards"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{id}/items
    pub fn get_by_id_items(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "items"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{id}/params/{param-key}/remapping
    pub fn get_by_id_params_by_param_key_remapping(
        &self,
        id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "dashboard",
            id.as_str(),
            "params",
            param_key.as_str(),
            "remapping",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{id}/params/{param-key}/search/{query}
    pub fn get_by_id_params_by_param_key_search_by_query(
        &self,
        id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "dashboard",
            id.as_str(),
            "params",
            param_key.as_str(),
            "search",
            query.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{id}/params/{param-key}/values
    pub fn get_by_id_params_by_param_key_values(
        &self,
        id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let id = id.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "dashboard",
            id.as_str(),
            "params",
            param_key.as_str(),
            "values",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{id}/query_metadata
    pub fn get_by_id_query_metadata(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "query_metadata"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/dashboard/{id}/related
    pub fn get_by_id_related(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "dashboard", id.as_str(), "related"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
