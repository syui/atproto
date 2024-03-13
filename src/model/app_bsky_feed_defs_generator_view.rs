use serde::{Serialize, Deserialize};
use super::{
    AppBskyActorDefsProfileView, AppBskyFeedDefsGeneratorViewerState,
    AppBskyRichtextFacet, ComAtprotoLabelDefsLabel,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedDefsGeneratorView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub cid: String,
    pub creator: AppBskyActorDefsProfileView,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionFacets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
    pub did: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    #[serde(rename = "likeCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    pub uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<AppBskyFeedDefsGeneratorViewerState>,
}
impl std::fmt::Display for AppBskyFeedDefsGeneratorView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}