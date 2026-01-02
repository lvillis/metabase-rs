use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct GeojsonService {
    client: Client,
}

#[cfg(feature = "async")]
impl GeojsonService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/geojson/
    pub async fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "geojson"];
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

    /// GET /api/geojson/{key}
    pub async fn get_by_key(&self, key: impl Into<PathParam>) -> Result<Value> {
        let key = key.into();
        let segments = ["api", "geojson", key.as_str()];
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
pub struct BlockingGeojsonService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingGeojsonService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/geojson/
    pub fn get(&self, query: Option<&Value>) -> Result<Value> {
        let segments = ["api", "geojson"];
        self.client.request_json(
            Method::GET,
            &segments,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/geojson/{key}
    pub fn get_by_key(&self, key: impl Into<PathParam>) -> Result<Value> {
        let key = key.into();
        let segments = ["api", "geojson", key.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
