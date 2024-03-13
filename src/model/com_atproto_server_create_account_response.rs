use serde::{Serialize, Deserialize};
///Account login session returned on successful account creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerCreateAccountResponse {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    ///The DID of the new account.
    pub did: String,
    #[serde(rename = "didDoc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<serde_json::Value>,
    pub handle: String,
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,
}
impl std::fmt::Display for ComAtprotoServerCreateAccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}