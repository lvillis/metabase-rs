use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct TimelineEventService {
    client: Client,
}

#[cfg(feature = "async")]
impl TimelineEventService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/timeline-event/
    pub async fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline-event"];
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

    /// DELETE /api/timeline-event/{id}
    pub async fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline-event", id.as_str()];
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

    /// GET /api/timeline-event/{id}
    pub async fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline-event", id.as_str()];
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

    /// PUT /api/timeline-event/{id}
    pub async fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline-event", id.as_str()];
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
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingTimelineEventService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingTimelineEventService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/timeline-event/
    pub fn post(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "timeline-event"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/timeline-event/{id}
    pub fn delete_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline-event", id.as_str()];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/timeline-event/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline-event", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/timeline-event/{id}
    pub fn put_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "timeline-event", id.as_str()];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
