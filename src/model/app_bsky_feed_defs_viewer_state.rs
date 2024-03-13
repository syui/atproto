use serde::{Serialize, Deserialize};
///Metadata about the requesting account's relationship with the subject content. Only has meaningful content for authed requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsViewerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like: Option<String>,
    #[serde(rename = "replyDisabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repost: Option<String>,
}
impl std::fmt::Display for AppBskyFeedDefsViewerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}