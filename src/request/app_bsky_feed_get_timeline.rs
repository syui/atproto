use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_feed_get_timeline`].

On request success, this will return a [`AppBskyFeedGetTimelineResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetTimelineRequest {
    pub algorithm: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}
impl AppBskyFeedGetTimelineRequest {}
impl FluentRequest<'_, AppBskyFeedGetTimelineRequest> {
    ///Set the value of the algorithm field.
    pub fn algorithm(mut self, algorithm: &str) -> Self {
        self.params.algorithm = Some(algorithm.to_owned());
        self
    }
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AppBskyFeedGetTimelineRequest> {
    type Output = httpclient::InMemoryResult<AppBskyFeedGetTimelineResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.feed.getTimeline";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}