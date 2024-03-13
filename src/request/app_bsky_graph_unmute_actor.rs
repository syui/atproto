use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_graph_unmute_actor`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphUnmuteActorRequest {
    pub actor: String,
}
impl AppBskyGraphUnmuteActorRequest {}
impl FluentRequest<'_, AppBskyGraphUnmuteActorRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyGraphUnmuteActorRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.graph.unmuteActor";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "actor" : self.params.actor }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}