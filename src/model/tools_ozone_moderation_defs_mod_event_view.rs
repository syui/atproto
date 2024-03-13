use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationDefsModEventView {
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "creatorHandle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_handle: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub event: serde_json::Value,
    pub id: i64,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub subject: serde_json::Value,
    #[serde(rename = "subjectBlobCids")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_blob_cids: Vec<String>,
    #[serde(rename = "subjectHandle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_handle: Option<String>,
}
impl std::fmt::Display for ToolsOzoneModerationDefsModEventView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}