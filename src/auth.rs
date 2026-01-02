use http::{HeaderMap, HeaderValue};
use secrecy::{ExposeSecret, SecretString};

use crate::error::Error;

/// Authentication methods for the Metabase API.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub enum Auth {
    /// No authentication (public endpoints only).
    #[default]
    None,
    /// Metabase session token (`X-Metabase-Session`).
    Session { token: SecretString },
    /// Metabase API key (typically `X-API-KEY`).
    ApiKey { key: SecretString },
}

impl Auth {
    /// No authentication (public endpoints only).
    pub fn none() -> Self {
        Self::None
    }

    /// Create an auth config from a Metabase session token.
    pub fn session(token: impl Into<String>) -> Self {
        Self::Session {
            token: SecretString::from(token.into()),
        }
    }

    /// Create an auth config from a Metabase API key.
    pub fn api_key(key: impl Into<String>) -> Self {
        Self::ApiKey {
            key: SecretString::from(key.into()),
        }
    }

    pub(crate) fn apply(&self, headers: &mut HeaderMap) -> Result<(), Error> {
        match self {
            Auth::None => Ok(()),
            Auth::Session { token } => {
                let value = HeaderValue::from_str(token.expose_secret())
                    .map_err(|source| Error::invalid_header_value("X-Metabase-Session", source))?;
                headers.insert("X-Metabase-Session", value);
                Ok(())
            }
            Auth::ApiKey { key } => {
                let value = HeaderValue::from_str(key.expose_secret())
                    .map_err(|source| Error::invalid_header_value("X-API-KEY", source))?;
                headers.insert("X-API-KEY", value);
                Ok(())
            }
        }
    }
}
