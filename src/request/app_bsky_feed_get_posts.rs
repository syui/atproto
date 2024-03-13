use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_feed_get_posts`].

On request success, this will return a [`AppBskyFeedGetPostsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetPostsRequest {
    pub uris: Vec<String>,
}
impl AppBskyFeedGetPostsRequest {}
impl FluentRequest<'_, AppBskyFeedGetPostsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AppBskyFeedGetPostsRequest> {
    type Output = httpclient::InMemoryResult<AppBskyFeedGetPostsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.feed.getPosts";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}