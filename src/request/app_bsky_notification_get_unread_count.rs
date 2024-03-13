use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_notification_get_unread_count`].

On request success, this will return a [`AppBskyNotificationGetUnreadCountResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationGetUnreadCountRequest {
    pub seen_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl AppBskyNotificationGetUnreadCountRequest {}
impl FluentRequest<'_, AppBskyNotificationGetUnreadCountRequest> {
    ///Set the value of the seen_at field.
    pub fn seen_at(mut self, seen_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.seen_at = Some(seen_at);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyNotificationGetUnreadCountRequest> {
    type Output = httpclient::InMemoryResult<AppBskyNotificationGetUnreadCountResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.notification.getUnreadCount";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}