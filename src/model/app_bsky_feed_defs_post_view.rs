use serde::{Serialize, Deserialize};
use super::{
    AppBskyActorDefsProfileViewBasic, AppBskyFeedDefsThreadgateView,
    AppBskyFeedDefsViewerState, ComAtprotoLabelDefsLabel,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsPostView {
    pub author: AppBskyActorDefsProfileViewBasic,
    pub cid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed: Option<serde_json::Value>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    #[serde(rename = "likeCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub record: serde_json::Value,
    #[serde(rename = "replyCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(rename = "repostCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repost_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threadgate: Option<AppBskyFeedDefsThreadgateView>,
    pub uri: String,
    ///Metadata about the requesting account's relationship with the subject content. Only has meaningful content for authed requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<AppBskyFeedDefsViewerState>,
}
impl std::fmt::Display for AppBskyFeedDefsPostView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}