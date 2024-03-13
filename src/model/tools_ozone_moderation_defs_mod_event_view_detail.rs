use serde::{Serialize, Deserialize};
use super::ToolsOzoneModerationDefsBlobView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationDefsModEventViewDetail {
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub event: serde_json::Value,
    pub id: i64,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub subject: serde_json::Value,
    #[serde(rename = "subjectBlobs")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_blobs: Vec<ToolsOzoneModerationDefsBlobView>,
}
impl std::fmt::Display for ToolsOzoneModerationDefsModEventViewDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}