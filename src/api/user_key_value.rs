use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct UserKeyValueService {
    client: Client,
}

#[cfg(feature = "async")]
impl UserKeyValueService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/user-key-value/namespace/{namespace}
    pub async fn get_namespace_by_namespace(
        &self,
        namespace: impl Into<PathParam>,
    ) -> Result<Value> {
        let namespace = namespace.into();
        let segments = ["api", "user-key-value", "namespace", namespace.as_str()];
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

    /// DELETE /api/user-key-value/namespace/{namespace}/key/{key}
    pub async fn delete_namespace_by_namespace_key_by_key(
        &self,
        namespace: impl Into<PathParam>,
        key: impl Into<PathParam>,
    ) -> Result<Value> {
        let namespace = namespace.into();
        let key = key.into();
        let segments = [
            "api",
            "user-key-value",
            "namespace",
            namespace.as_str(),
            "key",
            key.as_str(),
        ];
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

    /// GET /api/user-key-value/namespace/{namespace}/key/{key}
    pub async fn get_namespace_by_namespace_key_by_key(
        &self,
        namespace: impl Into<PathParam>,
        key: impl Into<PathParam>,
    ) -> Result<Value> {
        let namespace = namespace.into();
        let key = key.into();
        let segments = [
            "api",
            "user-key-value",
            "namespace",
            namespace.as_str(),
            "key",
            key.as_str(),
        ];
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

    /// PUT /api/user-key-value/namespace/{namespace}/key/{key}
    pub async fn put_namespace_by_namespace_key_by_key(
        &self,
        namespace: impl Into<PathParam>,
        key: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let namespace = namespace.into();
        let key = key.into();
        let segments = [
            "api",
            "user-key-value",
            "namespace",
            namespace.as_str(),
            "key",
            key.as_str(),
        ];
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
pub struct BlockingUserKeyValueService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingUserKeyValueService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/user-key-value/namespace/{namespace}
    pub fn get_namespace_by_namespace(&self, namespace: impl Into<PathParam>) -> Result<Value> {
        let namespace = namespace.into();
        let segments = ["api", "user-key-value", "namespace", namespace.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// DELETE /api/user-key-value/namespace/{namespace}/key/{key}
    pub fn delete_namespace_by_namespace_key_by_key(
        &self,
        namespace: impl Into<PathParam>,
        key: impl Into<PathParam>,
    ) -> Result<Value> {
        let namespace = namespace.into();
        let key = key.into();
        let segments = [
            "api",
            "user-key-value",
            "namespace",
            namespace.as_str(),
            "key",
            key.as_str(),
        ];
        self.client.request_json(
            Method::DELETE,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/user-key-value/namespace/{namespace}/key/{key}
    pub fn get_namespace_by_namespace_key_by_key(
        &self,
        namespace: impl Into<PathParam>,
        key: impl Into<PathParam>,
    ) -> Result<Value> {
        let namespace = namespace.into();
        let key = key.into();
        let segments = [
            "api",
            "user-key-value",
            "namespace",
            namespace.as_str(),
            "key",
            key.as_str(),
        ];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// PUT /api/user-key-value/namespace/{namespace}/key/{key}
    pub fn put_namespace_by_namespace_key_by_key(
        &self,
        namespace: impl Into<PathParam>,
        key: impl Into<PathParam>,
        body: Option<&Value>,
    ) -> Result<Value> {
        let namespace = namespace.into();
        let key = key.into();
        let segments = [
            "api",
            "user-key-value",
            "namespace",
            namespace.as_str(),
            "key",
            key.as_str(),
        ];
        self.client.request_json(
            Method::PUT,
            &segments,
            Option::<&()>::None,
            body,
            RequestOptions::default(),
        )
    }
}
