use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGetRepostedByResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "repostedBy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reposted_by: Vec<AppBskyActorDefsProfileView>,
    pub uri: String,
}
impl std::fmt::Display for AppBskyFeedGetRepostedByResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}