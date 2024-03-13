use serde::{Serialize, Deserialize};
use super::AppBskyGraphDefsListViewBasic;
///Metadata about the requesting account's relationship with the subject account. Only has meaningful content for authed requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsViewerState {
    #[serde(rename = "blockedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_by: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking: Option<String>,
    #[serde(rename = "blockingByList")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking_by_list: Option<AppBskyGraphDefsListViewBasic>,
    #[serde(rename = "followedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(rename = "mutedByList")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted_by_list: Option<AppBskyGraphDefsListViewBasic>,
}
impl std::fmt::Display for AppBskyActorDefsViewerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}