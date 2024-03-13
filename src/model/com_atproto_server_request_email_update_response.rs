use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerRequestEmailUpdateResponse {
    #[serde(rename = "tokenRequired")]
    pub token_required: bool,
}
impl std::fmt::Display for ComAtprotoServerRequestEmailUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}