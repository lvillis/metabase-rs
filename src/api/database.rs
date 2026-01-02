use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct DatabaseService {
    client: Client,
}

#[cfg(feature = "async")]
impl DatabaseService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/database/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "database"];
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

    /// POST /api/database/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "database"];
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

    /// POST /api/database/sample_database
    pub async fn post_sample_database(&self) -> Result<Value> {
        let segments = ["api", "database", "sample_database"];
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

    /// POST /api/database/validate
    pub async fn post_validate(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "database", "validate"];
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

    /// DELETE /api/database/{id}
    pub async fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str()];
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

    /// GET /api/database/{id}
    pub async fn get_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str()];
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

    /// PUT /api/database/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str()];
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

    /// GET /api/database/{id}/autocomplete_suggestions
    pub async fn get_by_id_autocomplete_suggestions(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "autocomplete_suggestions"];
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

    /// GET /api/database/{id}/card_autocomplete_suggestions
    pub async fn get_by_id_card_autocomplete_suggestions(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "database",
            id.as_str(),
            "card_autocomplete_suggestions",
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

    /// POST /api/database/{id}/discard_values
    pub async fn post_by_id_discard_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "discard_values"];
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

    /// POST /api/database/{id}/dismiss_spinner
    pub async fn post_by_id_dismiss_spinner(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "dismiss_spinner"];
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

    /// GET /api/database/{id}/fields
    pub async fn get_by_id_fields(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "fields"];
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

    /// GET /api/database/{id}/healthcheck
    pub async fn get_by_id_healthcheck(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "healthcheck"];
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

    /// GET /api/database/{id}/idfields
    pub async fn get_by_id_idfields(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "idfields"];
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

    /// GET /api/database/{id}/metadata
    pub async fn get_by_id_metadata(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "metadata"];
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

    /// POST /api/database/{id}/rescan_values
    pub async fn post_by_id_rescan_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "rescan_values"];
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

    /// GET /api/database/{id}/schema/
    pub async fn get_by_id_schema(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "schema"];
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

    /// GET /api/database/{id}/schema/{schema}
    pub async fn get_by_id_schema_by_schema(
        &self,
        id: impl Into<PathParam>,
        schema: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let schema = schema.into();
        let segments = ["api", "database", id.as_str(), "schema", schema.as_str()];
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

    /// GET /api/database/{id}/schemas
    pub async fn get_by_id_schemas(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "schemas"];
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

    /// GET /api/database/{id}/settings-available
    pub async fn get_by_id_settings_available(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "settings-available"];
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

    /// POST /api/database/{id}/sync_schema
    pub async fn post_by_id_sync_schema(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "sync_schema"];
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

    /// GET /api/database/{id}/syncable_schemas
    pub async fn get_by_id_syncable_schemas(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "syncable_schemas"];
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

    /// GET /api/database/{id}/usage_info
    pub async fn get_by_id_usage_info(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "usage_info"];
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

    /// GET /api/database/{virtual-db}/datasets
    pub async fn get_by_virtual_db_datasets(
        &self,
        virtual_db: impl Into<PathParam>,
    ) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let segments = ["api", "database", virtual_db.as_str(), "datasets"];
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

    /// GET /api/database/{virtual-db}/datasets/{schema}
    pub async fn get_by_virtual_db_datasets_by_schema(
        &self,
        virtual_db: impl Into<PathParam>,
        schema: impl Into<PathParam>,
    ) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let schema = schema.into();
        let segments = [
            "api",
            "database",
            virtual_db.as_str(),
            "datasets",
            schema.as_str(),
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

    /// GET /api/database/{virtual-db}/metadata
    pub async fn get_by_virtual_db_metadata(
        &self,
        virtual_db: impl Into<PathParam>,
    ) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let segments = ["api", "database", virtual_db.as_str(), "metadata"];
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

    /// GET /api/database/{virtual-db}/schema/{schema}
    pub async fn get_by_virtual_db_schema_by_schema(
        &self,
        virtual_db: impl Into<PathParam>,
        schema: impl Into<PathParam>,
    ) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let schema = schema.into();
        let segments = [
            "api",
            "database",
            virtual_db.as_str(),
            "schema",
            schema.as_str(),
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

    /// GET /api/database/{virtual-db}/schemas
    pub async fn get_by_virtual_db_schemas(
        &self,
        virtual_db: impl Into<PathParam>,
    ) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let segments = ["api", "database", virtual_db.as_str(), "schemas"];
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
pub struct BlockingDatabaseService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingDatabaseService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/database/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "database"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/database/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "database"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/database/sample_database
    pub fn post_sample_database(&self) -> Result<Value> {
        let segments = ["api", "database", "sample_database"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/database/validate
    pub fn post_validate(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "database", "validate"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/database/{id}
    pub fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>, query: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/database/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/autocomplete_suggestions
    pub fn get_by_id_autocomplete_suggestions(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "autocomplete_suggestions"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/card_autocomplete_suggestions
    pub fn get_by_id_card_autocomplete_suggestions(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = [
            "api",
            "database",
            id.as_str(),
            "card_autocomplete_suggestions",
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/database/{id}/discard_values
    pub fn post_by_id_discard_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "discard_values"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/database/{id}/dismiss_spinner
    pub fn post_by_id_dismiss_spinner(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "dismiss_spinner"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/fields
    pub fn get_by_id_fields(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "fields"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/healthcheck
    pub fn get_by_id_healthcheck(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "healthcheck"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/idfields
    pub fn get_by_id_idfields(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "idfields"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/metadata
    pub fn get_by_id_metadata(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "metadata"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/database/{id}/rescan_values
    pub fn post_by_id_rescan_values(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "rescan_values"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/schema/
    pub fn get_by_id_schema(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "schema"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/schema/{schema}
    pub fn get_by_id_schema_by_schema(
        &self,
        id: impl Into<PathParam>,
        schema: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let schema = schema.into();
        let segments = ["api", "database", id.as_str(), "schema", schema.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/schemas
    pub fn get_by_id_schemas(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "schemas"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/settings-available
    pub fn get_by_id_settings_available(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "settings-available"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/database/{id}/sync_schema
    pub fn post_by_id_sync_schema(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "sync_schema"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/syncable_schemas
    pub fn get_by_id_syncable_schemas(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "syncable_schemas"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{id}/usage_info
    pub fn get_by_id_usage_info(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "database", id.as_str(), "usage_info"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{virtual-db}/datasets
    pub fn get_by_virtual_db_datasets(&self, virtual_db: impl Into<PathParam>) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let segments = ["api", "database", virtual_db.as_str(), "datasets"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{virtual-db}/datasets/{schema}
    pub fn get_by_virtual_db_datasets_by_schema(
        &self,
        virtual_db: impl Into<PathParam>,
        schema: impl Into<PathParam>,
    ) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let schema = schema.into();
        let segments = [
            "api",
            "database",
            virtual_db.as_str(),
            "datasets",
            schema.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{virtual-db}/metadata
    pub fn get_by_virtual_db_metadata(&self, virtual_db: impl Into<PathParam>) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let segments = ["api", "database", virtual_db.as_str(), "metadata"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{virtual-db}/schema/{schema}
    pub fn get_by_virtual_db_schema_by_schema(
        &self,
        virtual_db: impl Into<PathParam>,
        schema: impl Into<PathParam>,
    ) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let schema = schema.into();
        let segments = [
            "api",
            "database",
            virtual_db.as_str(),
            "schema",
            schema.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/database/{virtual-db}/schemas
    pub fn get_by_virtual_db_schemas(&self, virtual_db: impl Into<PathParam>) -> Result<Value> {
        let virtual_db = virtual_db.into();
        let segments = ["api", "database", virtual_db.as_str(), "schemas"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
