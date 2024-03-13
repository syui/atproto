use serde::{Serialize, Deserialize};
use super::ComAtprotoModerationDefsReasonType;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoModerationCreateReportResponse {
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "reasonType")]
    pub reason_type: ComAtprotoModerationDefsReasonType,
    #[serde(rename = "reportedBy")]
    pub reported_by: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub subject: serde_json::Value,
}
impl std::fmt::Display for ComAtprotoModerationCreateReportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}