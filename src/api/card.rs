use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct CardService {
    client: Client,
}

#[cfg(feature = "async")]
impl CardService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/card/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "card"];
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

    /// POST /api/card/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "card"];
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

    /// POST /api/card/collections
    pub async fn post_collections(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "card", "collections"];
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

    /// GET /api/card/embeddable
    pub async fn get_embeddable(&self) -> Result<Value> {
        let segments = ["api", "card", "embeddable"];
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

    /// POST /api/card/pivot/{card-id}/query
    pub async fn post_pivot_by_card_id_query(
        &self,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", "pivot", card_id.as_str(), "query"];
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

    /// GET /api/card/public
    pub async fn get_public(&self) -> Result<Value> {
        let segments = ["api", "card", "public"];
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

    /// GET /api/card/{card-id}/params/{param-key}/search/{query}
    pub async fn get_by_card_id_params_by_param_key_search_by_query(
        &self,
        card_id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "card",
            card_id.as_str(),
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

    /// GET /api/card/{card-id}/params/{param-key}/values
    pub async fn get_by_card_id_params_by_param_key_values(
        &self,
        card_id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "card",
            card_id.as_str(),
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

    /// DELETE /api/card/{card-id}/public_link
    pub async fn delete_by_card_id_public_link(
        &self,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", card_id.as_str(), "public_link"];
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

    /// POST /api/card/{card-id}/public_link
    pub async fn post_by_card_id_public_link(
        &self,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", card_id.as_str(), "public_link"];
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

    /// POST /api/card/{card-id}/query
    pub async fn post_by_card_id_query(
        &self,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", card_id.as_str(), "query"];
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

    /// POST /api/card/{card-id}/query/{export-format}
    pub async fn post_by_card_id_query_by_export_format(
        &self,
        card_id: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let card_id = card_id.into();
        let export_format = export_format.into();
        let segments = [
            "api",
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

    /// DELETE /api/card/{id}
    pub async fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str()];
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

    /// GET /api/card/{id}
    pub async fn get_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str()];
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

    /// PUT /api/card/{id}
    pub async fn put_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str()];
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

    /// POST /api/card/{id}/copy
    pub async fn post_by_id_copy(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "copy"];
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

    /// GET /api/card/{id}/dashboards
    pub async fn get_by_id_dashboards(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "dashboards"];
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

    /// GET /api/card/{id}/params/{param-key}/remapping
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
            "card",
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

    /// GET /api/card/{id}/query_metadata
    pub async fn get_by_id_query_metadata(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "query_metadata"];
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

    /// GET /api/card/{id}/series
    pub async fn get_by_id_series(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "series"];
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
pub struct BlockingCardService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingCardService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/card/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "card"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/card/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "card"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/card/collections
    pub fn post_collections(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "card", "collections"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/card/embeddable
    pub fn get_embeddable(&self) -> Result<Value> {
        let segments = ["api", "card", "embeddable"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/card/pivot/{card-id}/query
    pub fn post_pivot_by_card_id_query(
        &self,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", "pivot", card_id.as_str(), "query"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/card/public
    pub fn get_public(&self) -> Result<Value> {
        let segments = ["api", "card", "public"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/card/{card-id}/params/{param-key}/search/{query}
    pub fn get_by_card_id_params_by_param_key_search_by_query(
        &self,
        card_id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
        query: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let param_key = param_key.into();
        let query = query.into();
        let segments = [
            "api",
            "card",
            card_id.as_str(),
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

    /// GET /api/card/{card-id}/params/{param-key}/values
    pub fn get_by_card_id_params_by_param_key_values(
        &self,
        card_id: impl Into<PathParam>,
        param_key: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let param_key = param_key.into();
        let segments = [
            "api",
            "card",
            card_id.as_str(),
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

    /// DELETE /api/card/{card-id}/public_link
    pub fn delete_by_card_id_public_link(&self, card_id: impl Into<PathParam>) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", card_id.as_str(), "public_link"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/card/{card-id}/public_link
    pub fn post_by_card_id_public_link(&self, card_id: impl Into<PathParam>) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", card_id.as_str(), "public_link"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/card/{card-id}/query
    pub fn post_by_card_id_query(
        &self,
        card_id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "card", card_id.as_str(), "query"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/card/{card-id}/query/{export-format}
    pub fn post_by_card_id_query_by_export_format(
        &self,
        card_id: impl Into<PathParam>,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let card_id = card_id.into();
        let export_format = export_format.into();
        let segments = [
            "api",
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

    /// DELETE /api/card/{id}
    pub fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/card/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>, query: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/card/{id}
    pub fn put_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            query,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/card/{id}/copy
    pub fn post_by_id_copy(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "copy"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/card/{id}/dashboards
    pub fn get_by_id_dashboards(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "dashboards"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/card/{id}/params/{param-key}/remapping
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
            "card",
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

    /// GET /api/card/{id}/query_metadata
    pub fn get_by_id_query_metadata(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "query_metadata"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/card/{id}/series
    pub fn get_by_id_series(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "card", id.as_str(), "series"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
