use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PreviewEmbedService {
    client: Client,
}

#[cfg(feature = "async")]
impl PreviewEmbedService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/preview_embed/card/{token}
    pub async fn get_card_by_token(&self, token: impl Into<PathParam>) -> Result<Value> {
        let token = token.into();
        let segments = ["api", "preview_embed", "card", token.as_str()];
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

    /// GET /api/preview_embed/card/{token}/params/{param-key}/remapping
    pub async fn get_card_by_token_params_by_param_key_remapping(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "preview_embed",
            "card",
            token.as_str(),
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

    /// GET /api/preview_embed/card/{token}/query
    pub async fn get_card_by_token_query(&self, token: impl Into<PathParam>) -> Result<Value> {
        let token = token.into();
        let segments = ["api", "preview_embed", "card", token.as_str(), "query"];
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

    /// GET /api/preview_embed/dashboard/{token}
    pub async fn get_dashboard_by_token(&self, token: impl Into<PathParam>) -> Result<Value> {
        let token = token.into();
        let segments = ["api", "preview_embed", "dashboard", token.as_str()];
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

    /// GET /api/preview_embed/dashboard/{token}/dashcard/{dashcard-id}/card/{card-id}
    pub async fn get_dashboard_by_token_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        token: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
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

    /// GET /api/preview_embed/dashboard/{token}/params/{param-key}/remapping
    pub async fn get_dashboard_by_token_params_by_param_key_remapping(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
            "params",
            param_key.as_str(),
            "remapping",
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

    /// GET /api/preview_embed/dashboard/{token}/params/{param-key}/search/{prefix}
    pub async fn get_dashboard_by_token_params_by_param_key_search_by_prefix(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let prefix = prefix.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
            "params",
            param_key.as_str(),
            "search",
            prefix.as_str(),
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

    /// GET /api/preview_embed/dashboard/{token}/params/{param-key}/values
    pub async fn get_dashboard_by_token_params_by_param_key_values(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
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

    /// GET /api/preview_embed/pivot/card/{token}/query
    pub async fn get_pivot_card_by_token_query(
        &self,
        token: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let segments = [
            "api",
            "preview_embed",
            "pivot",
            "card",
            token.as_str(),
            "query",
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

    /// GET /api/preview_embed/pivot/dashboard/{token}/dashcard/{dashcard-id}/card/{card-id}
    pub async fn get_pivot_dashboard_by_token_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        token: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "preview_embed",
            "pivot",
            "dashboard",
            token.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
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

    /// GET /api/preview_embed/tiles/card/{token}/{zoom}/{x}/{y}
    pub async fn get_tiles_card_by_token_by_zoom_by_x_by_y(
        &self,
        token: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let token = token.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "preview_embed",
            "tiles",
            "card",
            token.as_str(),
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

    /// GET /api/preview_embed/tiles/dashboard/{token}/dashcard/{dashcard-id}/card/{card-id}/{zoom}/{x}/{y}
    pub async fn get_tiles_dashboard_by_token_dashcard_by_dashcard_id_card_by_card_id_by_zoom_by_x_by_y(
        &self,
        token: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let token = token.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "preview_embed",
            "tiles",
            "dashboard",
            token.as_str(),
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
pub struct BlockingPreviewEmbedService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPreviewEmbedService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/preview_embed/card/{token}
    pub fn get_card_by_token(&self, token: impl Into<PathParam>) -> Result<Value> {
        let token = token.into();
        let segments = ["api", "preview_embed", "card", token.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/card/{token}/params/{param-key}/remapping
    pub fn get_card_by_token_params_by_param_key_remapping(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "preview_embed",
            "card",
            token.as_str(),
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

    /// GET /api/preview_embed/card/{token}/query
    pub fn get_card_by_token_query(&self, token: impl Into<PathParam>) -> Result<Value> {
        let token = token.into();
        let segments = ["api", "preview_embed", "card", token.as_str(), "query"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/dashboard/{token}
    pub fn get_dashboard_by_token(&self, token: impl Into<PathParam>) -> Result<Value> {
        let token = token.into();
        let segments = ["api", "preview_embed", "dashboard", token.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/dashboard/{token}/dashcard/{dashcard-id}/card/{card-id}
    pub fn get_dashboard_by_token_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        token: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/dashboard/{token}/params/{param-key}/remapping
    pub fn get_dashboard_by_token_params_by_param_key_remapping(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
            "params",
            param_key.as_str(),
            "remapping",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/dashboard/{token}/params/{param-key}/search/{prefix}
    pub fn get_dashboard_by_token_params_by_param_key_search_by_prefix(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        prefix: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let prefix = prefix.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
            "params",
            param_key.as_str(),
            "search",
            prefix.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/dashboard/{token}/params/{param-key}/values
    pub fn get_dashboard_by_token_params_by_param_key_values(
        &self,
        token: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "preview_embed",
            "dashboard",
            token.as_str(),
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

    /// GET /api/preview_embed/pivot/card/{token}/query
    pub fn get_pivot_card_by_token_query(&self, token: impl Into<PathParam>) -> Result<Value> {
        let token = token.into();
        let segments = [
            "api",
            "preview_embed",
            "pivot",
            "card",
            token.as_str(),
            "query",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/pivot/dashboard/{token}/dashcard/{dashcard-id}/card/{card-id}
    pub fn get_pivot_dashboard_by_token_dashcard_by_dashcard_id_card_by_card_id(
        &self,
        token: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let token = token.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let segments = [
            "api",
            "preview_embed",
            "pivot",
            "dashboard",
            token.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/preview_embed/tiles/card/{token}/{zoom}/{x}/{y}
    pub fn get_tiles_card_by_token_by_zoom_by_x_by_y(
        &self,
        token: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let token = token.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "preview_embed",
            "tiles",
            "card",
            token.as_str(),
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

    /// GET /api/preview_embed/tiles/dashboard/{token}/dashcard/{dashcard-id}/card/{card-id}/{zoom}/{x}/{y}
    pub fn get_tiles_dashboard_by_token_dashcard_by_dashcard_id_card_by_card_id_by_zoom_by_x_by_y(
        &self,
        token: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let token = token.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "preview_embed",
            "tiles",
            "dashboard",
            token.as_str(),
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
