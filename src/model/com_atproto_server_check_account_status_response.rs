use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCheckAccountStatusResponse {
    pub activated: bool,
    #[serde(rename = "expectedBlobs")]
    pub expected_blobs: i64,
    #[serde(rename = "importedBlobs")]
    pub imported_blobs: i64,
    #[serde(rename = "indexedRecords")]
    pub indexed_records: i64,
    #[serde(rename = "privateStateValues")]
    pub private_state_values: i64,
    #[serde(rename = "repoBlocks")]
    pub repo_blocks: i64,
    #[serde(rename = "repoCommit")]
    pub repo_commit: String,
    #[serde(rename = "repoRev")]
    pub repo_rev: String,
    #[serde(rename = "validDid")]
    pub valid_did: bool,
}
impl std::fmt::Display for ComAtprotoServerCheckAccountStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}