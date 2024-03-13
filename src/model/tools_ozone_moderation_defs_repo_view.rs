use serde::{Serialize, Deserialize};
use super::{ComAtprotoServerDefsInviteCode, ToolsOzoneModerationDefsModeration};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationDefsRepoView {
    pub did: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub handle: String,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "inviteNote")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,
    #[serde(rename = "invitedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
    #[serde(rename = "invitesDisabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invites_disabled: Option<bool>,
    pub moderation: ToolsOzoneModerationDefsModeration,
    #[serde(rename = "relatedRecords")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_records: Vec<serde_json::Value>,
}
impl std::fmt::Display for ToolsOzoneModerationDefsRepoView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}