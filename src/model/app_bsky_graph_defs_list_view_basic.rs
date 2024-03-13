use serde::{Serialize, Deserialize};
use super::{
    AppBskyGraphDefsListPurpose, AppBskyGraphDefsListViewerState,
    ComAtprotoLabelDefsLabel,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListViewBasic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub cid: String,
    #[serde(rename = "indexedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    pub name: String,
    pub purpose: AppBskyGraphDefsListPurpose,
    pub uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<AppBskyGraphDefsListViewerState>,
}
impl std::fmt::Display for AppBskyGraphDefsListViewBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}