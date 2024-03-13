use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_actor_get_profiles`].

On request success, this will return a [`AppBskyActorGetProfilesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorGetProfilesRequest {
    pub actors: Vec<String>,
}
impl AppBskyActorGetProfilesRequest {}
impl FluentRequest<'_, AppBskyActorGetProfilesRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyActorGetProfilesRequest> {
    type Output = httpclient::InMemoryResult<AppBskyActorGetProfilesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.actor.getProfiles";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}