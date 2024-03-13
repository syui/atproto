use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyLabelerGetServicesResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<serde_json::Value>,
}
impl std::fmt::Display for AppBskyLabelerGetServicesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}