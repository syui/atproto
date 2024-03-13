use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGetPostThreadResponse {
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub thread: serde_json::Value,
}
impl std::fmt::Display for AppBskyFeedGetPostThreadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}