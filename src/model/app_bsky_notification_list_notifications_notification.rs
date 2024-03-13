use serde::{Serialize, Deserialize};
use super::{AppBskyActorDefsProfileView, ComAtprotoLabelDefsLabel};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyNotificationListNotificationsNotification {
    pub author: AppBskyActorDefsProfileView,
    pub cid: String,
    #[serde(rename = "indexedAt")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "isRead")]
    pub is_read: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
    ///Expected values are 'like', 'repost', 'follow', 'mention', 'reply', and 'quote'.
    pub reason: String,
    #[serde(rename = "reasonSubject")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_subject: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub record: serde_json::Value,
    pub uri: String,
}
impl std::fmt::Display for AppBskyNotificationListNotificationsNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}