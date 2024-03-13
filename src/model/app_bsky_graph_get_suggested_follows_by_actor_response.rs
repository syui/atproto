use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphGetSuggestedFollowsByActorResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<AppBskyActorDefsProfileView>,
}
impl std::fmt::Display for AppBskyGraphGetSuggestedFollowsByActorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}