use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct UploadService {
    client: Client,
}

#[cfg(feature = "async")]
impl UploadService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/upload/csv
    pub async fn post_csv(&self, form: &crate::types::multipart::MultipartForm) -> Result<Value> {
        let segments = ["api", "upload", "csv"];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingUploadService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingUploadService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/upload/csv
    pub fn post_csv(&self, form: &crate::types::multipart::MultipartForm) -> Result<Value> {
        let segments = ["api", "upload", "csv"];
        self.client.request_multipart_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            form,
            RequestOptions::default(),
        )
    }
}
