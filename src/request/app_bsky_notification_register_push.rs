use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_notification_register_push`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyNotificationRegisterPushRequest {
    pub app_id: String,
    pub platform: String,
    pub service_did: String,
    pub token: String,
}
impl AppBskyNotificationRegisterPushRequest {}
pub struct AppBskyNotificationRegisterPushRequired<'a> {
    pub app_id: &'a str,
    pub platform: &'a str,
    pub service_did: &'a str,
    pub token: &'a str,
}
impl<'a> AppBskyNotificationRegisterPushRequired<'a> {}
impl FluentRequest<'_, AppBskyNotificationRegisterPushRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyNotificationRegisterPushRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.notification.registerPush";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "appId" : self.params.app_id }));
            r = r.json(json!({ "platform" : self.params.platform }));
            r = r.json(json!({ "serviceDid" : self.params.service_did }));
            r = r.json(json!({ "token" : self.params.token }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}