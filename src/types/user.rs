/// Response body from `GET /api/user/current`.
#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
pub struct User {
    pub id: i64,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
}
