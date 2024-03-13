use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoTempCheckSignupQueueResponse {
    pub activated: bool,
    #[serde(rename = "estimatedTimeMs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_time_ms: Option<i64>,
    #[serde(rename = "placeInQueue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_in_queue: Option<i64>,
}
impl std::fmt::Display for ComAtprotoTempCheckSignupQueueResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}