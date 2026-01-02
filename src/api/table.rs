use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct TableService {
    client: Client,
}

#[cfg(feature = "async")]
impl TableService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/table/
    pub async fn get(&self) -> Result<Value> {
        let segments = ["api", "table"];
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

    /// PUT /api/table/
    pub async fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "table"];
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

    /// GET /api/table/card__:id/fks
    pub async fn get_card_id_fks(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let seg_0 = format!("card__{}", id.as_str());
        let segments = ["api", "table", seg_0.as_str(), "fks"];
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

    /// GET /api/table/card__:id/query_metadata
    pub async fn get_card_id_query_metadata(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let seg_0 = format!("card__{}", id.as_str());
        let segments = ["api", "table", seg_0.as_str(), "query_metadata"];
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

    /// GET /api/table/{id}
    pub async fn get_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str()];
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

    /// PUT /api/table/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str()];
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

    /// POST /api/table/{id}/append-csv
    pub async fn post_by_id_append_csv(
        &self,
        id: impl Into<PathParam>,
        form: &crate::types::multipart::MultipartForm,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "append-csv"];
        self.client
            .request_multipart_json(
                Method::POST,
                &segments,
                Option::<&()>::None,
                form,
                RequestOptions::default(),
            )
            .await
    }

    /// POST /api/table/{id}/discard_values
    pub async fn post_by_id_discard_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "discard_values"];
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

    /// PUT /api/table/{id}/fields/order
    pub async fn put_by_id_fields_order(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "fields", "order"];
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

    /// GET /api/table/{id}/fks
    pub async fn get_by_id_fks(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "fks"];
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

    /// GET /api/table/{id}/query_metadata
    pub async fn get_by_id_query_metadata(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "query_metadata"];
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

    /// GET /api/table/{id}/related
    pub async fn get_by_id_related(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "related"];
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

    /// POST /api/table/{id}/replace-csv
    pub async fn post_by_id_replace_csv(
        &self,
        id: impl Into<PathParam>,
        form: &crate::types::multipart::MultipartForm,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "replace-csv"];
        self.client
            .request_multipart_json(
                Method::POST,
                &segments,
                Option::<&()>::None,
                form,
                RequestOptions::default(),
            )
            .await
    }

    /// POST /api/table/{id}/rescan_values
    pub async fn post_by_id_rescan_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "rescan_values"];
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

    /// POST /api/table/{id}/sync_schema
    pub async fn post_by_id_sync_schema(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "sync_schema"];
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

    /// GET /api/table/{table-id}/data
    pub async fn get_by_table_id_data(&self, table_id: impl Into<PathParam>) -> Result<Value> {
        let table_id = table_id.into();
        let segments = ["api", "table", table_id.as_str(), "data"];
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
pub struct BlockingTableService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingTableService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/table/
    pub fn get(&self) -> Result<Value> {
        let segments = ["api", "table"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/table/
    pub fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "table"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/table/card__:id/fks
    pub fn get_card_id_fks(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let seg_0 = format!("card__{}", id.as_str());
        let segments = ["api", "table", seg_0.as_str(), "fks"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/table/card__:id/query_metadata
    pub fn get_card_id_query_metadata(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let seg_0 = format!("card__{}", id.as_str());
        let segments = ["api", "table", seg_0.as_str(), "query_metadata"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/table/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>, query: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/table/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/table/{id}/append-csv
    pub fn post_by_id_append_csv(
        &self,
        id: impl Into<PathParam>,
        form: &crate::types::multipart::MultipartForm,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "append-csv"];
        self.client.request_multipart_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            form,
            RequestOptions::default(),
        )
    }

    /// POST /api/table/{id}/discard_values
    pub fn post_by_id_discard_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "discard_values"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/table/{id}/fields/order
    pub fn put_by_id_fields_order(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "fields", "order"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/table/{id}/fks
    pub fn get_by_id_fks(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "fks"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/table/{id}/query_metadata
    pub fn get_by_id_query_metadata(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "query_metadata"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/table/{id}/related
    pub fn get_by_id_related(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "related"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/table/{id}/replace-csv
    pub fn post_by_id_replace_csv(
        &self,
        id: impl Into<PathParam>,
        form: &crate::types::multipart::MultipartForm,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "replace-csv"];
        self.client.request_multipart_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            form,
            RequestOptions::default(),
        )
    }

    /// POST /api/table/{id}/rescan_values
    pub fn post_by_id_rescan_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "rescan_values"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/table/{id}/sync_schema
    pub fn post_by_id_sync_schema(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "table", id.as_str(), "sync_schema"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/table/{table-id}/data
    pub fn get_by_table_id_data(&self, table_id: impl Into<PathParam>) -> Result<Value> {
        let table_id = table_id.into();
        let segments = ["api", "table", table_id.as_str(), "data"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
