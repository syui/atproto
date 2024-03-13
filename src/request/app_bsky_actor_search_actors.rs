use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_actor_search_actors`].

On request success, this will return a [`AppBskyActorSearchActorsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyActorSearchActorsRequest {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub q: Option<String>,
    pub term: Option<String>,
}
impl AppBskyActorSearchActorsRequest {}
impl FluentRequest<'_, AppBskyActorSearchActorsRequest> {
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
    ///Set the value of the q field.
    pub fn q(mut self, q: &str) -> Self {
        self.params.q = Some(q.to_owned());
        self
    }
    ///Set the value of the term field.
    pub fn term(mut self, term: &str) -> Self {
        self.params.term = Some(term.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyActorSearchActorsRequest> {
    type Output = httpclient::InMemoryResult<AppBskyActorSearchActorsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.actor.searchActors";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}