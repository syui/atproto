use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_actor_get_profile`].

On request success, this will return a [`AppBskyActorDefsProfileViewDetailed`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetProfileRequest {
    pub actor: String,
}
impl AppBskyActorGetProfileRequest {}
impl FluentRequest<'_, AppBskyActorGetProfileRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AppBskyActorGetProfileRequest> {
    type Output = httpclient::InMemoryResult<AppBskyActorDefsProfileViewDetailed>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.actor.getProfile";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}