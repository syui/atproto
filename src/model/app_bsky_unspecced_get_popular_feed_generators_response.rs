use serde::{Serialize, Deserialize};
use super::AppBskyFeedDefsGeneratorView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyUnspeccedGetPopularFeedGeneratorsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
}
impl std::fmt::Display for AppBskyUnspeccedGetPopularFeedGeneratorsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}