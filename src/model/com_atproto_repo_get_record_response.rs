use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoRepoGetRecordResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    pub uri: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub value: serde_json::Value,
}
impl std::fmt::Display for ComAtprotoRepoGetRecordResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}