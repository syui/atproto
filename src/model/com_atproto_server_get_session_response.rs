use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerGetSessionResponse {
    pub did: String,
    #[serde(rename = "didDoc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "emailConfirmed")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_confirmed: Option<bool>,
    pub handle: String,
}
impl std::fmt::Display for ComAtprotoServerGetSessionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}