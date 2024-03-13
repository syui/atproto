use serde::{Serialize, Deserialize};
use super::ComAtprotoServerDefsInviteCodeUse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDefsInviteCode {
    pub available: i64,
    pub code: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    pub disabled: bool,
    #[serde(rename = "forAccount")]
    pub for_account: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uses: Vec<ComAtprotoServerDefsInviteCodeUse>,
}
impl std::fmt::Display for ComAtprotoServerDefsInviteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}