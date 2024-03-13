use serde::{Serialize, Deserialize};
use super::AppBskyGraphDefsListView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphGetListBlocksResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lists: Vec<AppBskyGraphDefsListView>,
}
impl std::fmt::Display for AppBskyGraphGetListBlocksResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}