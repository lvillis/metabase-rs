use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct AutomagicDashboardsService {
    client: Client,
}

#[cfg(feature = "async")]
impl AutomagicDashboardsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/automagic-dashboards/database/{id}/candidates
    pub async fn get_database_by_id_candidates(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "automagic-dashboards",
            "database",
            id.as_str(),
            "candidates",
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

    /// GET /api/automagic-dashboards/model_index/{model-index-id}/primary_key/{pk-id}
    pub async fn get_model_index_by_model_index_id_primary_key_by_pk_id(
        &self,
        model_index_id: impl Into<PathParam>,
        pk_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let model_index_id = model_index_id.into();
        let pk_id = pk_id.into();
        let segments = [
            "api",
            "automagic-dashboards",
            "model_index",
            model_index_id.as_str(),
            "primary_key",
            pk_id.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}
    pub async fn get_by_entity_by_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}
    pub async fn get_by_entity_by_entity_id_or_query_cell_by_cell_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub async fn get_by_entity_by_entity_id_or_query_cell_by_cell_query_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}/rule/{prefix}/{dashboard-template}
    pub async fn get_by_entity_by_entity_id_or_query_cell_by_cell_query_rule_by_prefix_by_dashboard_template(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}/rule/{prefix}/{dashboard-template}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub async fn get_by_entity_by_entity_id_or_query_cell_by_cell_query_rule_by_prefix_by_dashboard_template_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub async fn get_by_entity_by_entity_id_or_query_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/query_metadata
    pub async fn get_by_entity_by_entity_id_or_query_query_metadata(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "query_metadata",
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/rule/{prefix}/{dashboard-template}
    pub async fn get_by_entity_by_entity_id_or_query_rule_by_prefix_by_dashboard_template(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
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

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/rule/{prefix}/{dashboard-template}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub async fn get_by_entity_by_entity_id_or_query_rule_by_prefix_by_dashboard_template_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingAutomagicDashboardsService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingAutomagicDashboardsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/automagic-dashboards/database/{id}/candidates
    pub fn get_database_by_id_candidates(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "automagic-dashboards",
            "database",
            id.as_str(),
            "candidates",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/model_index/{model-index-id}/primary_key/{pk-id}
    pub fn get_model_index_by_model_index_id_primary_key_by_pk_id(
        &self,
        model_index_id: impl Into<PathParam>,
        pk_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let model_index_id = model_index_id.into();
        let pk_id = pk_id.into();
        let segments = [
            "api",
            "automagic-dashboards",
            "model_index",
            model_index_id.as_str(),
            "primary_key",
            pk_id.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}
    pub fn get_by_entity_by_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}
    pub fn get_by_entity_by_entity_id_or_query_cell_by_cell_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub fn get_by_entity_by_entity_id_or_query_cell_by_cell_query_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}/rule/{prefix}/{dashboard-template}
    pub fn get_by_entity_by_entity_id_or_query_cell_by_cell_query_rule_by_prefix_by_dashboard_template(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/cell/{cell-query}/rule/{prefix}/{dashboard-template}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub fn get_by_entity_by_entity_id_or_query_cell_by_cell_query_rule_by_prefix_by_dashboard_template_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        cell_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let cell_query = cell_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "cell",
            cell_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub fn get_by_entity_by_entity_id_or_query_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/query_metadata
    pub fn get_by_entity_by_entity_id_or_query_query_metadata(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "query_metadata",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/rule/{prefix}/{dashboard-template}
    pub fn get_by_entity_by_entity_id_or_query_rule_by_prefix_by_dashboard_template(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/automagic-dashboards/{entity}/{entity-id-or-query}/rule/{prefix}/{dashboard-template}/compare/{comparison-entity}/{comparison-entity-id-or-query}
    pub fn get_by_entity_by_entity_id_or_query_rule_by_prefix_by_dashboard_template_compare_by_comparison_entity_by_comparison_entity_id_or_query(
        &self,
        entity: impl Into<PathParam>,
        entity_id_or_query: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
        dashboard_template: impl Into<PathParam>,
        comparison_entity: impl Into<PathParam>,
        comparison_entity_id_or_query: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let entity = entity.into();
        let entity_id_or_query = entity_id_or_query.into();
        let prefix = prefix.into();
        let dashboard_template = dashboard_template.into();
        let comparison_entity = comparison_entity.into();
        let comparison_entity_id_or_query = comparison_entity_id_or_query.into();
        let segments = [
            "api",
            "automagic-dashboards",
            entity.as_str(),
            entity_id_or_query.as_str(),
            "rule",
            prefix.as_str(),
            dashboard_template.as_str(),
            "compare",
            comparison_entity.as_str(),
            comparison_entity_id_or_query.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
