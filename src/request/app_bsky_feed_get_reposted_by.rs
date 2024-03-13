use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_feed_get_reposted_by`].

On request success, this will return a [`AppBskyFeedGetRepostedByResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyFeedGetRepostedByRequest {
    pub cid: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub uri: String,
}
impl AppBskyFeedGetRepostedByRequest {}
impl FluentRequest<'_, AppBskyFeedGetRepostedByRequest> {
    ///Set the value of the cid field.
    pub fn cid(mut self, cid: &str) -> Self {
        self.params.cid = Some(cid.to_owned());
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
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyFeedGetRepostedByRequest> {
    type Output = httpclient::InMemoryResult<AppBskyFeedGetRepostedByResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.feed.getRepostedBy";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}