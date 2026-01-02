use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct ActivityService {
    client: Client,
}

#[cfg(feature = "async")]
impl ActivityService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/activity/most_recently_viewed_dashboard
    pub async fn get_most_recently_viewed_dashboard(&self) -> Result<Value> {
        let segments = ["api", "activity", "most_recently_viewed_dashboard"];
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

    /// GET /api/activity/popular_items
    pub async fn get_popular_items(&self) -> Result<Value> {
        let segments = ["api", "activity", "popular_items"];
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

    /// GET /api/activity/recent_views
    pub async fn get_recent_views(&self) -> Result<Value> {
        let segments = ["api", "activity", "recent_views"];
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

    /// GET /api/activity/recents
    pub async fn get_recents(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "activity", "recents"];
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

    /// POST /api/activity/recents
    pub async fn post_recents(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "activity", "recents"];
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
pub struct BlockingActivityService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingActivityService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/activity/most_recently_viewed_dashboard
    pub fn get_most_recently_viewed_dashboard(&self) -> Result<Value> {
        let segments = ["api", "activity", "most_recently_viewed_dashboard"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/activity/popular_items
    pub fn get_popular_items(&self) -> Result<Value> {
        let segments = ["api", "activity", "popular_items"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/activity/recent_views
    pub fn get_recent_views(&self) -> Result<Value> {
        let segments = ["api", "activity", "recent_views"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/activity/recents
    pub fn get_recents(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "activity", "recents"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/activity/recents
    pub fn post_recents(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "activity", "recents"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
