use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerRefreshSessionResponse {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    pub did: String,
    #[serde(rename = "didDoc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<serde_json::Value>,
    pub handle: String,
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,
}
impl std::fmt::Display for ComAtprotoServerRefreshSessionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}