use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoDescribeRepoResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub collections: Vec<String>,
    pub did: String,
    #[serde(rename = "didDoc")]
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub did_doc: serde_json::Value,
    pub handle: String,
    ///Indicates if handle is currently valid (resolves bi-directionally)
    #[serde(rename = "handleIsCorrect")]
    pub handle_is_correct: bool,
}
impl std::fmt::Display for ComAtprotoRepoDescribeRepoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}