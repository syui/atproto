use serde::{Serialize, Deserialize};
use super::ComAtprotoServerListAppPasswordsAppPassword;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerListAppPasswordsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<ComAtprotoServerListAppPasswordsAppPassword>,
}
impl std::fmt::Display for ComAtprotoServerListAppPasswordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}