use serde::{Serialize, Deserialize};
use super::AppBskyFeedDefsPostView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGetPostsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub posts: Vec<AppBskyFeedDefsPostView>,
}
impl std::fmt::Display for AppBskyFeedGetPostsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}