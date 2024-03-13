use serde::{Serialize, Deserialize};
use super::{
    ComAtprotoLabelDefsLabel, ComAtprotoServerDefsInviteCode,
    ToolsOzoneModerationDefsModerationDetail,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationDefsRepoViewDetail {
    pub did: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "emailConfirmedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub handle: String,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "inviteNote")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,
    #[serde(rename = "invitedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<ComAtprotoServerDefsInviteCode>>,
    #[serde(rename = "invitesDisabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invites_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    pub moderation: ToolsOzoneModerationDefsModerationDetail,
    #[serde(rename = "relatedRecords")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_records: Vec<serde_json::Value>,
}
impl std::fmt::Display for ToolsOzoneModerationDefsRepoViewDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}