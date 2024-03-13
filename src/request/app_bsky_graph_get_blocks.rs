use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_graph_get_blocks`].

On request success, this will return a [`AppBskyGraphGetBlocksResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyGraphGetBlocksRequest {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}
impl AppBskyGraphGetBlocksRequest {}
impl FluentRequest<'_, AppBskyGraphGetBlocksRequest> {
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AppBskyGraphGetBlocksRequest> {
    type Output = httpclient::InMemoryResult<AppBskyGraphGetBlocksResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.graph.getBlocks";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}