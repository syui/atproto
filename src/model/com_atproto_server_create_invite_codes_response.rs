use serde::{Serialize, Deserialize};
use super::ComAtprotoServerCreateInviteCodesAccountCodes;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCreateInviteCodesResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<ComAtprotoServerCreateInviteCodesAccountCodes>,
}
impl std::fmt::Display for ComAtprotoServerCreateInviteCodesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}