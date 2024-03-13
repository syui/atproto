use serde::{Serialize, Deserialize};
use super::{
    ComAtprotoLabelDefsLabel, ToolsOzoneModerationDefsBlobView,
    ToolsOzoneModerationDefsModerationDetail, ToolsOzoneModerationDefsRepoView,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationDefsRecordViewDetail {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blobs: Vec<ToolsOzoneModerationDefsBlobView>,
    pub cid: String,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    pub moderation: ToolsOzoneModerationDefsModerationDetail,
    pub repo: ToolsOzoneModerationDefsRepoView,
    pub uri: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub value: serde_json::Value,
}
impl std::fmt::Display for ToolsOzoneModerationDefsRecordViewDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}