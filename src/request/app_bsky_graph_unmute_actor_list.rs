use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_graph_unmute_actor_list`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphUnmuteActorListRequest {
    pub list: String,
}
impl AppBskyGraphUnmuteActorListRequest {}
impl FluentRequest<'_, AppBskyGraphUnmuteActorListRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyGraphUnmuteActorListRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.graph.unmuteActorList";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "list" : self.params.list }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}