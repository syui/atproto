use serde::{Serialize, Deserialize};
use super::AppBskyActorDefsProfileView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListItemView {
    pub subject: AppBskyActorDefsProfileView,
    pub uri: String,
}
impl std::fmt::Display for AppBskyGraphDefsListItemView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}