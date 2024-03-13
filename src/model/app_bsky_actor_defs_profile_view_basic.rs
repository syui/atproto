use serde::{Serialize, Deserialize};
use super::{AppBskyActorDefsViewerState, ComAtprotoLabelDefsLabel};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileViewBasic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub did: String,
    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub handle: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    ///Metadata about the requesting account's relationship with the subject account. Only has meaningful content for authed requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<AppBskyActorDefsViewerState>,
}
impl std::fmt::Display for AppBskyActorDefsProfileViewBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}