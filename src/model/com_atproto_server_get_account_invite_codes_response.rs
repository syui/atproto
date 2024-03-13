use serde::{Serialize, Deserialize};
use super::ComAtprotoServerDefsInviteCode;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerGetAccountInviteCodesResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<ComAtprotoServerDefsInviteCode>,
}
impl std::fmt::Display for ComAtprotoServerGetAccountInviteCodesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}