use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphGetMutesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutes: Vec<AppBskyActorDefsProfileView>,
}
impl std::fmt::Display for AppBskyGraphGetMutesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}