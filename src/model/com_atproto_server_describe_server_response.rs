use serde::{Serialize, Deserialize};
use super::ComAtprotoServerDescribeServerLinks;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDescribeServerResponse {
    #[serde(rename = "availableUserDomains")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_user_domains: Vec<String>,
    pub did: String,
    ///If true, an invite code must be supplied to create an account on this instance.
    #[serde(rename = "inviteCodeRequired")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_code_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ComAtprotoServerDescribeServerLinks>,
    ///If true, a phone verification token must be supplied to create an account on this instance.
    #[serde(rename = "phoneVerificationRequired")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_verification_required: Option<bool>,
}
impl std::fmt::Display for ComAtprotoServerDescribeServerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}