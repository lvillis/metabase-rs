use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PublicService {
    client: Client,
}

#[cfg(feature = "async")]
impl PublicService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/public/action/{uuid}
    pub async fn get_action_by_uuid(&self, uuid: impl Into<PathParam>) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "action", uuid.as_str()];
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

    /// POST /api/public/action/{uuid}/execute
    pub async fn post_action_by_uuid_execute(
        &self,
        uuid: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "action", uuid.as_str(), "execute"];
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

    /// GET /api/public/card/{uuid}
    pub async fn get_card_by_uuid(&self, uuid: impl Into<PathParam>) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "card", uuid.as_str()];
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

    /// GET /api/public/card/{uuid}/params/{param-key}/remapping
    pub async fn get_card_by_uuid_params_by_param_key_remapping(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
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

    /// GET /api/public/card/{uuid}/params/{param-key}/search/{query}
    pub async fn get_card_by_uuid_params_by_param_key_search_by_query(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
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

    /// GET /api/public/card/{uuid}/params/{param-key}/values
    pub async fn get_card_by_uuid_params_by_param_key_values(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
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

    /// GET /api/public/card/{uuid}/query
    pub async fn get_card_by_uuid_query(
        &self,
        uuid: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "card", uuid.as_str(), "query"];
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

    /// GET /api/public/card/{uuid}/query/{export-format}
    pub async fn get_card_by_uuid_query_by_export_format(
        &self,
        uuid: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let export_format = export_format.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
            "query",
            export_format.as_str(),
        ];
        self.client
            .request_bytes(
                Method::GET,
                &segments,
                query,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/public/dashboard/{uuid}
    pub async fn get_dashboard_by_uuid(&self, uuid: impl Into<PathParam>) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "dashboard", uuid.as_str()];
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

    /// GET /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}
    pub async fn get_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
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

    /// POST /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}/{export-format}
    pub async fn post_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id_by_export_format(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let export_format = export_format.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/execute
    pub async fn get_dashboard_by_uuid_dashcard_by_dashcard_id_execute(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// POST /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/execute
    pub async fn post_dashboard_by_uuid_dashcard_by_dashcard_id_execute(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/params/{param-key}/remapping
    pub async fn get_dashboard_by_uuid_params_by_param_key_remapping(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/params/{param-key}/search/{query}
    pub async fn get_dashboard_by_uuid_params_by_param_key_search_by_query(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/params/{param-key}/values
    pub async fn get_dashboard_by_uuid_params_by_param_key_values(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/oembed
    pub async fn get_oembed(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "public", "oembed"];
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

    /// GET /api/public/pivot/card/{uuid}/query
    pub async fn get_pivot_card_by_uuid_query(
        &self,
        uuid: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "pivot", "card", uuid.as_str(), "query"];
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

    /// GET /api/public/pivot/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}
    pub async fn get_pivot_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "public",
            "pivot",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
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

    /// GET /api/public/tiles/card/{uuid}/{zoom}/{x}/{y}
    pub async fn get_tiles_card_by_uuid_by_zoom_by_x_by_y(
        &self,
        uuid: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "public",
            "tiles",
            "card",
            uuid.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client
            .request_bytes(
                Method::GET,
                &segments,
                query,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/public/tiles/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}/{zoom}/{x}/{y}
    pub async fn get_tiles_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id_by_zoom_by_x_by_y(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "public",
            "tiles",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client
            .request_bytes(
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
pub struct BlockingPublicService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPublicService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/public/action/{uuid}
    pub fn get_action_by_uuid(&self, uuid: impl Into<PathParam>) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "action", uuid.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/public/action/{uuid}/execute
    pub fn post_action_by_uuid_execute(
        &self,
        uuid: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "action", uuid.as_str(), "execute"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/card/{uuid}
    pub fn get_card_by_uuid(&self, uuid: impl Into<PathParam>) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "card", uuid.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/card/{uuid}/params/{param-key}/remapping
    pub fn get_card_by_uuid_params_by_param_key_remapping(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
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

    /// GET /api/public/card/{uuid}/params/{param-key}/search/{query}
    pub fn get_card_by_uuid_params_by_param_key_search_by_query(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
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

    /// GET /api/public/card/{uuid}/params/{param-key}/values
    pub fn get_card_by_uuid_params_by_param_key_values(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
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

    /// GET /api/public/card/{uuid}/query
    pub fn get_card_by_uuid_query(
        &self,
        uuid: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "card", uuid.as_str(), "query"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/card/{uuid}/query/{export-format}
    pub fn get_card_by_uuid_query_by_export_format(
        &self,
        uuid: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let export_format = export_format.into();
        let segments = [
            "api",
            "public",
            "card",
            uuid.as_str(),
            "query",
            export_format.as_str(),
        ];
        self.client.request_bytes(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/dashboard/{uuid}
    pub fn get_dashboard_by_uuid(&self, uuid: impl Into<PathParam>) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "dashboard", uuid.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}
    pub fn get_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}/{export-format}
    pub fn post_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id_by_export_format(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let export_format = export_format.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/execute
    pub fn get_dashboard_by_uuid_dashcard_by_dashcard_id_execute(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// POST /api/public/dashboard/{uuid}/dashcard/{dashcard-id}/execute
    pub fn post_dashboard_by_uuid_dashcard_by_dashcard_id_execute(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/params/{param-key}/remapping
    pub fn get_dashboard_by_uuid_params_by_param_key_remapping(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/params/{param-key}/search/{query}
    pub fn get_dashboard_by_uuid_params_by_param_key_search_by_query(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/dashboard/{uuid}/params/{param-key}/values
    pub fn get_dashboard_by_uuid_params_by_param_key_values(
        &self,
        uuid: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "public",
            "dashboard",
            uuid.as_str(),
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

    /// GET /api/public/oembed
    pub fn get_oembed(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "public", "oembed"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/pivot/card/{uuid}/query
    pub fn get_pivot_card_by_uuid_query(
        &self,
        uuid: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let segments = ["api", "public", "pivot", "card", uuid.as_str(), "query"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/pivot/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}
    pub fn get_pivot_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "public",
            "pivot",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/tiles/card/{uuid}/{zoom}/{x}/{y}
    pub fn get_tiles_card_by_uuid_by_zoom_by_x_by_y(
        &self,
        uuid: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "public",
            "tiles",
            "card",
            uuid.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client.request_bytes(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/public/tiles/dashboard/{uuid}/dashcard/{dashcard-id}/card/{card-id}/{zoom}/{x}/{y}
    pub fn get_tiles_dashboard_by_uuid_dashcard_by_dashcard_id_card_by_card_id_by_zoom_by_x_by_y(
        &self,
        uuid: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let uuid = uuid.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "public",
            "tiles",
            "dashboard",
            uuid.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client.request_bytes(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
