use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyUnspeccedGetTaggedSuggestionsSuggestion {
    pub subject: String,
    #[serde(rename = "subjectType")]
    pub subject_type: String,
    pub tag: String,
}
impl std::fmt::Display for AppBskyUnspeccedGetTaggedSuggestionsSuggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}