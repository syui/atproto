use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsReplyRef {
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub parent: serde_json::Value,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub root: serde_json::Value,
}
impl std::fmt::Display for AppBskyFeedDefsReplyRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}