use serde::{Serialize, Deserialize};
use super::ToolsOzoneModerationDefsSubjectStatusView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationQueryStatusesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subjectStatuses")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_statuses: Vec<ToolsOzoneModerationDefsSubjectStatusView>,
}
impl std::fmt::Display for ToolsOzoneModerationQueryStatusesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}