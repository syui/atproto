use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_actor_put_preferences`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorPutPreferencesRequest {
    pub preferences: AppBskyActorDefsPreferences,
}
impl AppBskyActorPutPreferencesRequest {}
impl FluentRequest<'_, AppBskyActorPutPreferencesRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyActorPutPreferencesRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.actor.putPreferences";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "preferences" : self.params.preferences }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}