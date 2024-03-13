use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_notification_list_notifications`].

On request success, this will return a [`AppBskyNotificationListNotificationsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationListNotificationsRequest {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub seen_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl AppBskyNotificationListNotificationsRequest {}
impl FluentRequest<'_, AppBskyNotificationListNotificationsRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the limit field.
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    ///Set the value of the seen_at field.
    pub fn seen_at(mut self, seen_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.seen_at = Some(seen_at);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyNotificationListNotificationsRequest> {
    type Output = httpclient::InMemoryResult<
        AppBskyNotificationListNotificationsResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.notification.listNotifications";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}