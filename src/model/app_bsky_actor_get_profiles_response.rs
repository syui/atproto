use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileViewDetailed;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorGetProfilesResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<AppBskyActorDefsProfileViewDetailed>,
}
impl std::fmt::Display for AppBskyActorGetProfilesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}