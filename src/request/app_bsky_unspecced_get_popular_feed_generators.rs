use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_unspecced_get_popular_feed_generators`].

On request success, this will return a [`AppBskyUnspeccedGetPopularFeedGeneratorsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGetPopularFeedGeneratorsRequest {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub query: Option<String>,
}
impl AppBskyUnspeccedGetPopularFeedGeneratorsRequest {}
impl FluentRequest<'_, AppBskyUnspeccedGetPopularFeedGeneratorsRequest> {
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
    ///Set the value of the query field.
    pub fn query(mut self, query: &str) -> Self {
        self.params.query = Some(query.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyUnspeccedGetPopularFeedGeneratorsRequest> {
    type Output = httpclient::InMemoryResult<
        AppBskyUnspeccedGetPopularFeedGeneratorsResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.unspecced.getPopularFeedGenerators";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}