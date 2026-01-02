use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct EmailService {
    client: Client,
}

#[cfg(feature = "async")]
impl EmailService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// DELETE /api/email/
    pub async fn delete(&self) -> Result<Value> {
        let segments = ["api", "email"];
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

    /// PUT /api/email/
    pub async fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "email"];
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

    /// POST /api/email/test
    pub async fn post_test(&self) -> Result<Value> {
        let segments = ["api", "email", "test"];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingEmailService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingEmailService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// DELETE /api/email/
    pub fn delete(&self) -> Result<Value> {
        let segments = ["api", "email"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/email/
    pub fn put(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "email"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/email/test
    pub fn post_test(&self) -> Result<Value> {
        let segments = ["api", "email", "test"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
