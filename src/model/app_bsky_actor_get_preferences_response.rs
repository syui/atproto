use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsPreferences;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorGetPreferencesResponse {
    pub preferences: AppBskyActorDefsPreferences,
}
impl std::fmt::Display for AppBskyActorGetPreferencesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}