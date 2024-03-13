use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGetLikesLike {
    pub actor: AppBskyActorDefsProfileView,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for AppBskyFeedGetLikesLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}