use secrecy::{ExposeSecret, SecretString};

/// Request body for `POST /api/session`.
#[derive(Clone, Debug, serde::Serialize)]
#[non_exhaustive]
pub struct CreateSessionRequest {
    pub username: String,
    #[serde(serialize_with = "serialize_secret_string")]
    pub password: SecretString,
}

impl CreateSessionRequest {
    pub fn new(username: impl Into<String>, password: SecretString) -> Self {
        Self {
            username: username.into(),
            password,
        }
    }
}

fn serialize_secret_string<S>(secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

/// Response from `POST /api/session`.
#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
pub struct CreateSessionResponse {
    pub id: String,
}
