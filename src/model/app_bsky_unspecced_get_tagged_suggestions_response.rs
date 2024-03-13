use serde::{Serialize, Deserialize};
use super::AppBskyUnspeccedGetTaggedSuggestionsSuggestion;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyUnspeccedGetTaggedSuggestionsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<AppBskyUnspeccedGetTaggedSuggestionsSuggestion>,
}
impl std::fmt::Display for AppBskyUnspeccedGetTaggedSuggestionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}