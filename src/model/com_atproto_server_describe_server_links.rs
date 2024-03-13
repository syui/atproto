use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerDescribeServerLinks {
    #[serde(rename = "privacyPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    #[serde(rename = "termsOfService")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
}
impl std::fmt::Display for ComAtprotoServerDescribeServerLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}