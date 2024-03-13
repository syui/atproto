use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListViewerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
}
impl std::fmt::Display for AppBskyGraphDefsListViewerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}