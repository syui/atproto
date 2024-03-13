use serde::{Serialize, Deserialize};
use super::{
    AppBskyActorDefsProfileAssociated, AppBskyActorDefsViewerState,
    ComAtprotoLabelDefsLabel,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileViewDetailed {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated: Option<AppBskyActorDefsProfileAssociated>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub did: String,
    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "followersCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i64>,
    #[serde(rename = "followsCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follows_count: Option<i64>,
    pub handle: String,
    #[serde(rename = "indexedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    #[serde(rename = "postsCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
    ///Metadata about the requesting account's relationship with the subject account. Only has meaningful content for authed requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<AppBskyActorDefsViewerState>,
}
impl std::fmt::Display for AppBskyActorDefsProfileViewDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}