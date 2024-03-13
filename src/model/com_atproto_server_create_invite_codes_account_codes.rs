use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCreateInviteCodesAccountCodes {
    pub account: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<String>,
}
impl std::fmt::Display for ComAtprotoServerCreateInviteCodesAccountCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}