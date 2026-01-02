use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct DatasetService {
    client: Client,
}

#[cfg(feature = "async")]
impl DatasetService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/dataset/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset"];
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

    /// POST /api/dataset/native
    pub async fn post_native(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "native"];
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

    /// POST /api/dataset/parameter/remapping
    pub async fn post_parameter_remapping(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "parameter", "remapping"];
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

    /// POST /api/dataset/parameter/search/{query}
    pub async fn post_parameter_search_by_query(
        &self,
        query: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let query = query.into();
        let segments = ["api", "dataset", "parameter", "search", query.as_str()];
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

    /// POST /api/dataset/parameter/values
    pub async fn post_parameter_values(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "parameter", "values"];
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

    /// POST /api/dataset/pivot
    pub async fn post_pivot(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "pivot"];
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

    /// POST /api/dataset/query_metadata
    pub async fn post_query_metadata(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "query_metadata"];
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

    /// POST /api/dataset/{export-format}
    pub async fn post_by_export_format(
        &self,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let export_format = export_format.into();
        let segments = ["api", "dataset", export_format.as_str()];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingDatasetService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingDatasetService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/dataset/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dataset/native
    pub fn post_native(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "native"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dataset/parameter/remapping
    pub fn post_parameter_remapping(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "parameter", "remapping"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dataset/parameter/search/{query}
    pub fn post_parameter_search_by_query(
        &self,
        query: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let query = query.into();
        let segments = ["api", "dataset", "parameter", "search", query.as_str()];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dataset/parameter/values
    pub fn post_parameter_values(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "parameter", "values"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dataset/pivot
    pub fn post_pivot(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "pivot"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dataset/query_metadata
    pub fn post_query_metadata(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "dataset", "query_metadata"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/dataset/{export-format}
    pub fn post_by_export_format(
        &self,
        export_format: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let export_format = export_format.into();
        let segments = ["api", "dataset", export_format.as_str()];
        self.client.request_bytes(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
