use serde::{Serialize, Deserialize};
use super::AppBskyGraphDefsListViewBasic;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsThreadgateView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<AppBskyGraphDefsListViewBasic>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl std::fmt::Display for AppBskyFeedDefsThreadgateView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}