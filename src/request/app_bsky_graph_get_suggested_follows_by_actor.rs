use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_graph_get_suggested_follows_by_actor`].

On request success, this will return a [`AppBskyGraphGetSuggestedFollowsByActorResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetSuggestedFollowsByActorRequest {
    pub actor: String,
}
impl AppBskyGraphGetSuggestedFollowsByActorRequest {}
impl FluentRequest<'_, AppBskyGraphGetSuggestedFollowsByActorRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyGraphGetSuggestedFollowsByActorRequest> {
    type Output = httpclient::InMemoryResult<
        AppBskyGraphGetSuggestedFollowsByActorResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.graph.getSuggestedFollowsByActor";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}