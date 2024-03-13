use serde::{Serialize, Deserialize};
use super::ComAtprotoSyncListReposRepo;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncListReposResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repos: Vec<ComAtprotoSyncListReposRepo>,
}
impl std::fmt::Display for ComAtprotoSyncListReposResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}