use serde::{Serialize, Deserialize};
use super::AppBskyFeedDefsPostView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedSearchPostsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "hitsTotal")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits_total: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub posts: Vec<AppBskyFeedDefsPostView>,
}
impl std::fmt::Display for AppBskyFeedSearchPostsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}