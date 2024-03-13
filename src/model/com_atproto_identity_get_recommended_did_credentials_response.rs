use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoIdentityGetRecommendedDidCredentialsResponse {
    #[serde(rename = "alsoKnownAs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
    #[serde(rename = "rotationKeys")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<serde_json::Value>,
    #[serde(rename = "verificationMethods")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_methods: Option<serde_json::Value>,
}
impl std::fmt::Display for ComAtprotoIdentityGetRecommendedDidCredentialsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}