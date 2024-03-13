use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDefsInviteCodeUse {
    #[serde(rename = "usedAt")]
    pub used_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "usedBy")]
    pub used_by: String,
}
impl std::fmt::Display for ComAtprotoServerDefsInviteCodeUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}