use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct LdapService {
    client: Client,
}

#[cfg(feature = "async")]
impl LdapService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// PUT /api/ldap/settings
    pub async fn put_settings(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "ldap", "settings"];
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
pub struct BlockingLdapService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingLdapService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// PUT /api/ldap/settings
    pub fn put_settings(&self, body: Option<&Value>) -> Result<Value> {
        let segments = ["api", "ldap", "settings"];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
