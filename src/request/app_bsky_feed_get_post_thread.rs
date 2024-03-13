use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_feed_get_post_thread`].

On request success, this will return a [`AppBskyFeedGetPostThreadResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetPostThreadRequest {
    pub depth: Option<i64>,
    pub parent_height: Option<i64>,
    pub uri: String,
}
impl AppBskyFeedGetPostThreadRequest {}
impl FluentRequest<'_, AppBskyFeedGetPostThreadRequest> {
    ///Set the value of the depth field.
    pub fn depth(mut self, depth: i64) -> Self {
        self.params.depth = Some(depth);
        self
    }
    ///Set the value of the parent_height field.
    pub fn parent_height(mut self, parent_height: i64) -> Self {
        self.params.parent_height = Some(parent_height);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyFeedGetPostThreadRequest> {
    type Output = httpclient::InMemoryResult<AppBskyFeedGetPostThreadResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.feed.getPostThread";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}