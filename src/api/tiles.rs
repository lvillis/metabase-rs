use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct TilesService {
    client: Client,
}

#[cfg(feature = "async")]
impl TilesService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/tiles/{card-id}/{zoom}/{x}/{y}
    pub async fn get_by_card_id_by_zoom_by_x_by_y(
        &self,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "tiles",
            card_id.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client
            .request_bytes(
                Method::GET,
                &segments,
                query,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/tiles/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/{zoom}/{x}/{y}
    pub async fn get_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_by_zoom_by_x_by_y(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "tiles",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client
            .request_bytes(
                Method::GET,
                &segments,
                query,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/tiles/{zoom}/{x}/{y}
    pub async fn get_by_zoom_by_x_by_y(
        &self,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = ["api", "tiles", zoom.as_str(), x.as_str(), y.as_str()];
        self.client
            .request_bytes(
                Method::GET,
                &segments,
                query,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingTilesService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingTilesService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/tiles/{card-id}/{zoom}/{x}/{y}
    pub fn get_by_card_id_by_zoom_by_x_by_y(
        &self,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "tiles",
            card_id.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client.request_bytes(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/tiles/{dashboard-id}/dashcard/{dashcard-id}/card/{card-id}/{zoom}/{x}/{y}
    pub fn get_by_dashboard_id_dashcard_by_dashcard_id_card_by_card_id_by_zoom_by_x_by_y(
        &self,
        dashboard_id: impl Into<PathParam>,
        dashcard_id: impl Into<PathParam>,
        card_id: impl Into<PathParam>,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let dashboard_id = dashboard_id.into();
        let dashcard_id = dashcard_id.into();
        let card_id = card_id.into();
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = [
            "api",
            "tiles",
            dashboard_id.as_str(),
            "dashcard",
            dashcard_id.as_str(),
            "card",
            card_id.as_str(),
            zoom.as_str(),
            x.as_str(),
            y.as_str(),
        ];
        self.client.request_bytes(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/tiles/{zoom}/{x}/{y}
    pub fn get_by_zoom_by_x_by_y(
        &self,
        zoom: impl Into<PathParam>,
        x: impl Into<PathParam>,
        y: impl Into<PathParam>,
        query: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let zoom = zoom.into();
        let x = x.into();
        let y = y.into();
        let segments = ["api", "tiles", zoom.as_str(), x.as_str(), y.as_str()];
        self.client.request_bytes(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
