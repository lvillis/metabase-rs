/// A passthrough type for large, evolving JSON payloads.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(transparent)]
pub struct Dashboard(pub serde_json::Value);
