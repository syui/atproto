use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_unspecced_get_popular`].

On request success, this will return a [`AppBskyUnspeccedGetPopularResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGetPopularRequest {
    pub cursor: Option<String>,
    pub include_nsfw: Option<bool>,
    pub limit: Option<i64>,
}
impl AppBskyUnspeccedGetPopularRequest {}
impl FluentRequest<'_, AppBskyUnspeccedGetPopularRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the include_nsfw field.
    pub fn include_nsfw(mut self, include_nsfw: bool) -> Self {
        self.params.include_nsfw = Some(include_nsfw);
        self
    }
    ///Set the value of the limit field.
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyUnspeccedGetPopularRequest> {
    type Output = httpclient::InMemoryResult<AppBskyUnspeccedGetPopularResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.unspecced.getPopular";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}