use serde::{Serialize, Deserialize};
use super::ToolsOzoneModerationDefsSubjectStatusView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationDefsModeration {
    #[serde(rename = "subjectStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_status: Option<ToolsOzoneModerationDefsSubjectStatusView>,
}
impl std::fmt::Display for ToolsOzoneModerationDefsModeration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}