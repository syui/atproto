use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_feed_get_feed_generator`].

On request success, this will return a [`AppBskyFeedGetFeedGeneratorResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetFeedGeneratorRequest {
    pub feed: String,
}
impl AppBskyFeedGetFeedGeneratorRequest {}
impl FluentRequest<'_, AppBskyFeedGetFeedGeneratorRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyFeedGetFeedGeneratorRequest> {
    type Output = httpclient::InMemoryResult<AppBskyFeedGetFeedGeneratorResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.feed.getFeedGenerator";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}