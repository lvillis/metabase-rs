/// Response from `GET /api/health`.
#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
pub struct HealthResponse {
    pub status: String,
}
