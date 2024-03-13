use serde::{Serialize, Deserialize};
use super::AppBskyNotificationListNotificationsNotification;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyNotificationListNotificationsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<AppBskyNotificationListNotificationsNotification>,
    #[serde(rename = "seenAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for AppBskyNotificationListNotificationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}