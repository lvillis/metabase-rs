use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PulseService {
    client: Client,
}

#[cfg(feature = "async")]
impl PulseService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/pulse/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse"];
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

    /// POST /api/pulse/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse"];
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

    /// GET /api/pulse/form_input
    pub async fn get_form_input(&self) -> Result<Value> {
        let segments = ["api", "pulse", "form_input"];
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

    /// GET /api/pulse/preview_card/{id}
    pub async fn get_preview_card_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_card", id.as_str()];
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

    /// GET /api/pulse/preview_card_info/{id}
    pub async fn get_preview_card_info_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_card_info", id.as_str()];
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

    /// GET /api/pulse/preview_card_png/{id}
    pub async fn get_preview_card_png_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_card_png", id.as_str()];
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

    /// GET /api/pulse/preview_dashboard/{id}
    pub async fn get_preview_dashboard_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_dashboard", id.as_str()];
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

    /// POST /api/pulse/test
    pub async fn post_test(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse", "test"];
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

    /// GET /api/pulse/{id}
    pub async fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", id.as_str()];
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

    /// PUT /api/pulse/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", id.as_str()];
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

    /// DELETE /api/pulse/{id}/subscription
    pub async fn delete_by_id_subscription(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", id.as_str(), "subscription"];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingPulseService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPulseService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/pulse/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/pulse/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/pulse/form_input
    pub fn get_form_input(&self) -> Result<Value> {
        let segments = ["api", "pulse", "form_input"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/pulse/preview_card/{id}
    pub fn get_preview_card_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_card", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/pulse/preview_card_info/{id}
    pub fn get_preview_card_info_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_card_info", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/pulse/preview_card_png/{id}
    pub fn get_preview_card_png_by_id(
        &self,
        id: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_card_png", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/pulse/preview_dashboard/{id}
    pub fn get_preview_dashboard_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", "preview_dashboard", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/pulse/test
    pub fn post_test(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "pulse", "test"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/pulse/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/pulse/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/pulse/{id}/subscription
    pub fn delete_by_id_subscription(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "pulse", id.as_str(), "subscription"];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
