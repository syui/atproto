use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileViewBasic;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorSearchActorsTypeaheadResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actors: Vec<AppBskyActorDefsProfileViewBasic>,
}
impl std::fmt::Display for AppBskyActorSearchActorsTypeaheadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}