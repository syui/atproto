use serde::{Serialize, Deserialize};
use super::{
    AppBskyActorDefsProfileView, AppBskyGraphDefsListPurpose,
    AppBskyGraphDefsListViewerState, AppBskyRichtextFacet, ComAtprotoLabelDefsLabel,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub cid: String,
    pub creator: AppBskyActorDefsProfileView,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionFacets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    pub name: String,
    pub purpose: AppBskyGraphDefsListPurpose,
    pub uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<AppBskyGraphDefsListViewerState>,
}
impl std::fmt::Display for AppBskyGraphDefsListView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}