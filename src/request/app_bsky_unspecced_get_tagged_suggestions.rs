use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_unspecced_get_tagged_suggestions`].

On request success, this will return a [`AppBskyUnspeccedGetTaggedSuggestionsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedGetTaggedSuggestionsRequest {}
impl AppBskyUnspeccedGetTaggedSuggestionsRequest {}
impl FluentRequest<'_, AppBskyUnspeccedGetTaggedSuggestionsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyUnspeccedGetTaggedSuggestionsRequest> {
    type Output = httpclient::InMemoryResult<
        AppBskyUnspeccedGetTaggedSuggestionsResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.unspecced.getTaggedSuggestions";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}