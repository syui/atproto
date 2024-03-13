use serde::{Serialize, Deserialize};
use super::AppBskyFeedDefsGeneratorView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedGetFeedGeneratorResponse {
    ///Indicates whether the feed generator service has been online recently, or else seems to be inactive.
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    ///Indicates whether the feed generator service is compatible with the record declaration.
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    pub view: AppBskyFeedDefsGeneratorView,
}
impl std::fmt::Display for AppBskyFeedGetFeedGeneratorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}