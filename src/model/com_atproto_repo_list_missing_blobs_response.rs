use serde::{Serialize, Deserialize};
use super::ComAtprotoRepoListMissingBlobsRecordBlob;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoListMissingBlobsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blobs: Vec<ComAtprotoRepoListMissingBlobsRecordBlob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
impl std::fmt::Display for ComAtprotoRepoListMissingBlobsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}