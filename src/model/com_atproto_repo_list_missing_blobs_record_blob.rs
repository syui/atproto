use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoListMissingBlobsRecordBlob {
    pub cid: String,
    #[serde(rename = "recordUri")]
    pub record_uri: String,
}
impl std::fmt::Display for ComAtprotoRepoListMissingBlobsRecordBlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}