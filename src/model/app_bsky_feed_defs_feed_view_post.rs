use serde::{Serialize, Deserialize};
use super::{AppBskyFeedDefsPostView, AppBskyFeedDefsReplyRef};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsFeedViewPost {
    pub post: AppBskyFeedDefsPostView,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: Option<AppBskyFeedDefsReplyRef>,
}
impl std::fmt::Display for AppBskyFeedDefsFeedViewPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}