use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_feed_get_feed_generators`].

On request success, this will return a [`AppBskyFeedGetFeedGeneratorsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetFeedGeneratorsRequest {
    pub feeds: Vec<String>,
}
impl AppBskyFeedGetFeedGeneratorsRequest {}
impl FluentRequest<'_, AppBskyFeedGetFeedGeneratorsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyFeedGetFeedGeneratorsRequest> {
    type Output = httpclient::InMemoryResult<AppBskyFeedGetFeedGeneratorsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.feed.getFeedGenerators";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}