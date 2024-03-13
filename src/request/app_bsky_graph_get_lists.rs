use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_graph_get_lists`].

On request success, this will return a [`AppBskyGraphGetListsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetListsRequest {
    pub actor: String,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}
impl AppBskyGraphGetListsRequest {}
impl FluentRequest<'_, AppBskyGraphGetListsRequest> {
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
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AppBskyGraphGetListsRequest> {
    type Output = httpclient::InMemoryResult<AppBskyGraphGetListsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.graph.getLists";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}