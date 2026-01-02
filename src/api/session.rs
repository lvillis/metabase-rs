use http::Method;
use serde_json::Value;

use crate::types::session::{CreateSessionRequest, CreateSessionResponse};
use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct SessionService {
    client: Client,
}

#[cfg(feature = "async")]
impl SessionService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateSessionRequest) -> Result<CreateSessionResponse> {
        let segments = ["api", "session"];
        self.client.post_json(&segments, request).await
    }

    pub async fn create_with_options(
        &self,
        request: &CreateSessionRequest,
        options: RequestOptions,
    ) -> Result<CreateSessionResponse> {
        let segments = ["api", "session"];
        self.client
            .post_json_with_options(&segments, request, options)
            .await
    }

    /// DELETE /api/session/
    pub async fn delete(&self) -> Result<Value> {
        let segments = ["api", "session"];
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

    /// POST /api/session/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session"];
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

    /// POST /api/session/forgot_password
    pub async fn post_forgot_password(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "forgot_password"];
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

    /// POST /api/session/google_auth
    pub async fn post_google_auth(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "google_auth"];
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

    /// POST /api/session/password-check
    pub async fn post_password_check(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "password-check"];
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

    /// GET /api/session/password_reset_token_valid
    pub async fn get_password_reset_token_valid(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "password_reset_token_valid"];
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

    /// GET /api/session/properties
    pub async fn get_properties(&self) -> Result<Value> {
        let segments = ["api", "session", "properties"];
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

    /// POST /api/session/reset_password
    pub async fn post_reset_password(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "reset_password"];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingSessionService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingSessionService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn create(&self, request: &CreateSessionRequest) -> Result<CreateSessionResponse> {
        let segments = ["api", "session"];
        self.client.post_json(&segments, request)
    }

    pub fn create_with_options(
        &self,
        request: &CreateSessionRequest,
        options: RequestOptions,
    ) -> Result<CreateSessionResponse> {
        let segments = ["api", "session"];
        self.client
            .post_json_with_options(&segments, request, options)
    }

    /// DELETE /api/session/
    pub fn delete(&self) -> Result<Value> {
        let segments = ["api", "session"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/session/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/session/forgot_password
    pub fn post_forgot_password(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "forgot_password"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/session/google_auth
    pub fn post_google_auth(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "google_auth"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/session/password-check
    pub fn post_password_check(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "password-check"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/session/password_reset_token_valid
    pub fn get_password_reset_token_valid(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "password_reset_token_valid"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/session/properties
    pub fn get_properties(&self) -> Result<Value> {
        let segments = ["api", "session", "properties"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/session/reset_password
    pub fn post_reset_password(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "session", "reset_password"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
