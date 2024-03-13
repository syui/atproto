use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_notification_update_seen`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationUpdateSeenRequest {
    pub seen_at: chrono::DateTime<chrono::Utc>,
}
impl AppBskyNotificationUpdateSeenRequest {}
impl FluentRequest<'_, AppBskyNotificationUpdateSeenRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyNotificationUpdateSeenRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.notification.updateSeen";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "seenAt" : self.params.seen_at }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}