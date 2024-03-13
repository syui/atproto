use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorGetSuggestionsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actors: Vec<AppBskyActorDefsProfileView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
impl std::fmt::Display for AppBskyActorGetSuggestionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}