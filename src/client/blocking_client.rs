use std::sync::Arc;

use http::Method;
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::{
    Result,
    auth::Auth,
    client::{ClientConfig, RequestOptions},
    error::Error,
    transport::BlockingTransport,
    types::multipart::MultipartForm,
    util,
};

#[derive(Clone)]
pub struct BlockingClient {
    inner: Arc<Inner>,
}

struct Inner {
    base_url: Url,
    auth: Auth,
    transport: BlockingTransport,
}

pub struct BlockingClientBuilder {
    base_url: Url,
    auth: Auth,
    config: ClientConfig,
}

impl BlockingClient {
    pub fn builder(base_url: impl AsRef<str>) -> Result<BlockingClientBuilder> {
        let base_url_str = base_url.as_ref().to_owned();
        let base_url = Url::parse(&base_url_str)
            .map_err(|source| Error::invalid_base_url(base_url_str.clone(), source))?;
        if base_url.cannot_be_a_base() {
            return Err(Error::invalid_base_url_config(
                base_url_str.clone(),
                "base_url must be hierarchical",
            ));
        }
        if base_url.query().is_some() {
            return Err(Error::invalid_base_url_config(
                base_url_str.clone(),
                "base_url must not include a query string",
            ));
        }
        if base_url.fragment().is_some() {
            return Err(Error::invalid_base_url_config(
                base_url_str.clone(),
                "base_url must not include a fragment",
            ));
        }
        let base_url = util::normalize_base_url(base_url);

        Ok(BlockingClientBuilder {
            base_url,
            auth: Auth::none(),
            config: ClientConfig::default(),
        })
    }

    pub fn with_auth(&self, auth: Auth) -> Self {
        Self {
            inner: Arc::new(Inner {
                base_url: self.inner.base_url.clone(),
                auth,
                transport: self.inner.transport.clone(),
            }),
        }
    }

    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    pub(crate) fn request_json<T, Q, B>(
        &self,
        method: Method,
        path: &[&str],
        query: Option<&Q>,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize,
    {
        let mut url = util::build_url(&self.inner.base_url, path)?;
        util::set_query(&mut url, query)?;
        self.inner
            .transport
            .execute_json(method, url, &self.inner.auth, body, options)
    }

    pub(crate) fn request_bytes<Q, B>(
        &self,
        method: Method,
        path: &[&str],
        query: Option<&Q>,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<Vec<u8>>
    where
        Q: Serialize + ?Sized,
        B: Serialize,
    {
        let mut url = util::build_url(&self.inner.base_url, path)?;
        util::set_query(&mut url, query)?;
        self.inner
            .transport
            .execute_bytes(method, url, &self.inner.auth, body, options)
    }

    pub(crate) fn request_multipart_json<T, Q>(
        &self,
        method: Method,
        path: &[&str],
        query: Option<&Q>,
        form: &MultipartForm,
        options: RequestOptions,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let mut url = util::build_url(&self.inner.base_url, path)?;
        util::set_query(&mut url, query)?;
        self.inner
            .transport
            .execute_multipart_json(method, url, &self.inner.auth, form, options)
    }

    pub(crate) fn get_json<T, Q>(&self, path: &[&str], query: Option<&Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        self.request_json(
            Method::GET,
            path,
            query,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    pub(crate) fn post_json<T, B>(&self, path: &[&str], body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_json(
            Method::POST,
            path,
            Option::<&()>::None,
            Some(body),
            RequestOptions::default(),
        )
    }

    pub(crate) fn post_json_with_options<T, B>(
        &self,
        path: &[&str],
        body: &B,
        options: RequestOptions,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_json(Method::POST, path, Option::<&()>::None, Some(body), options)
    }
}

impl BlockingClientBuilder {
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn connect_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeouts.connect = timeout;
        self
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeouts.request = timeout;
        self
    }

    pub fn read_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeouts.read = timeout;
        self
    }

    pub fn capture_body_snippet(mut self, enabled: bool) -> Self {
        self.config.body_snippet.capture = enabled;
        self
    }

    pub fn body_snippet_limit(mut self, limit: usize) -> Self {
        self.config.body_snippet.limit = limit;
        self
    }

    pub fn redact_body_snippet(mut self, enabled: bool) -> Self {
        self.config.body_snippet.redact = enabled;
        self
    }

    pub fn retry_policy(mut self, policy: crate::client::RetryPolicy) -> Self {
        self.config.retry = policy;
        self
    }

    pub fn build(self) -> Result<BlockingClient> {
        let transport = BlockingTransport::new(&self.config)?;
        Ok(BlockingClient {
            inner: Arc::new(Inner {
                base_url: self.base_url,
                auth: self.auth,
                transport,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_rejects_query_and_fragment() -> crate::Result<()> {
        let err = match BlockingClient::builder("https://example.com/?a=b") {
            Ok(_) => return Err(crate::Error::invalid_base_url_config("unexpected", "ok")),
            Err(err) => err,
        };
        assert!(matches!(
            err,
            crate::Error::InvalidBaseUrlConfig {
                reason: "base_url must not include a query string",
                ..
            }
        ));

        let err = match BlockingClient::builder("https://example.com/#frag") {
            Ok(_) => return Err(crate::Error::invalid_base_url_config("unexpected", "ok")),
            Err(err) => err,
        };
        assert!(matches!(
            err,
            crate::Error::InvalidBaseUrlConfig {
                reason: "base_url must not include a fragment",
                ..
            }
        ));

        Ok(())
    }
}
