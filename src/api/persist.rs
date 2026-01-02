use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PersistService {
    client: Client,
}

#[cfg(feature = "async")]
impl PersistService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/persist/
    pub async fn get(&self) -> Result<Value> {
        let segments = ["api", "persist"];
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

    /// GET /api/persist/card/{card-id}
    pub async fn get_card_by_card_id(&self, card_id: impl Into<PathParam>) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str()];
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

    /// POST /api/persist/card/{card-id}/persist
    pub async fn post_card_by_card_id_persist(
        &self,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str(), "persist"];
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

    /// POST /api/persist/card/{card-id}/refresh
    pub async fn post_card_by_card_id_refresh(
        &self,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str(), "refresh"];
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

    /// POST /api/persist/card/{card-id}/unpersist
    pub async fn post_card_by_card_id_unpersist(
        &self,
        card_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str(), "unpersist"];
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

    /// POST /api/persist/database/{id}/persist
    pub async fn post_database_by_id_persist(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "persist", "database", id.as_str(), "persist"];
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

    /// POST /api/persist/database/{id}/unpersist
    pub async fn post_database_by_id_unpersist(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "persist", "database", id.as_str(), "unpersist"];
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

    /// POST /api/persist/disable
    pub async fn post_disable(&self) -> Result<Value> {
        let segments = ["api", "persist", "disable"];
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

    /// POST /api/persist/enable
    pub async fn post_enable(&self) -> Result<Value> {
        let segments = ["api", "persist", "enable"];
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

    /// POST /api/persist/set-refresh-schedule
    pub async fn post_set_refresh_schedule(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "persist", "set-refresh-schedule"];
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

    /// GET /api/persist/{persisted-info-id}
    pub async fn get_by_persisted_info_id(
        &self,
        persisted_info_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let persisted_info_id = persisted_info_id.into();
        let segments = ["api", "persist", persisted_info_id.as_str()];
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
pub struct BlockingPersistService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPersistService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/persist/
    pub fn get(&self) -> Result<Value> {
        let segments = ["api", "persist"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/persist/card/{card-id}
    pub fn get_card_by_card_id(&self, card_id: impl Into<PathParam>) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/card/{card-id}/persist
    pub fn post_card_by_card_id_persist(&self, card_id: impl Into<PathParam>) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str(), "persist"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/card/{card-id}/refresh
    pub fn post_card_by_card_id_refresh(&self, card_id: impl Into<PathParam>) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str(), "refresh"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/card/{card-id}/unpersist
    pub fn post_card_by_card_id_unpersist(&self, card_id: impl Into<PathParam>) -> Result<Value> {
        let card_id = card_id.into();
        let segments = ["api", "persist", "card", card_id.as_str(), "unpersist"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/database/{id}/persist
    pub fn post_database_by_id_persist(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "persist", "database", id.as_str(), "persist"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/database/{id}/unpersist
    pub fn post_database_by_id_unpersist(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "persist", "database", id.as_str(), "unpersist"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/disable
    pub fn post_disable(&self) -> Result<Value> {
        let segments = ["api", "persist", "disable"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/enable
    pub fn post_enable(&self) -> Result<Value> {
        let segments = ["api", "persist", "enable"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// POST /api/persist/set-refresh-schedule
    pub fn post_set_refresh_schedule(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "persist", "set-refresh-schedule"];
        self.client.request_json(
            Method::POST,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }

    /// GET /api/persist/{persisted-info-id}
    pub fn get_by_persisted_info_id(
        &self,
        persisted_info_id: impl Into<PathParam>,
    ) -> Result<Value> {
        let persisted_info_id = persisted_info_id.into();
        let segments = ["api", "persist", persisted_info_id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
