use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct NotifyService {
    client: Client,
}

#[cfg(feature = "async")]
impl NotifyService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// POST /api/notify/db/attached_datawarehouse
    pub async fn post_db_attached_datawarehouse(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "notify", "db", "attached_datawarehouse"];
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

    /// POST /api/notify/db/{id}
    pub async fn post_db_by_id(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "notify", "db", id.as_str()];
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

    /// POST /api/notify/db/{id}/new-table
    pub async fn post_db_by_id_new_table(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "notify", "db", id.as_str(), "new-table"];
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
pub struct BlockingNotifyService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingNotifyService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// POST /api/notify/db/attached_datawarehouse
    pub fn post_db_attached_datawarehouse(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "notify", "db", "attached_datawarehouse"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/notify/db/{id}
    pub fn post_db_by_id(&self, id: impl Into<PathParam>, body: Option<&Value>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "notify", "db", id.as_str()];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// POST /api/notify/db/{id}/new-table
    pub fn post_db_by_id_new_table(
        &self,
        id: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "notify", "db", id.as_str(), "new-table"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
