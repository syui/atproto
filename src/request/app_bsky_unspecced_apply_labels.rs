use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_unspecced_apply_labels`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyUnspeccedApplyLabelsRequest {
    pub labels: Vec<ComAtprotoLabelDefsLabel>,
}
impl AppBskyUnspeccedApplyLabelsRequest {}
impl FluentRequest<'_, AppBskyUnspeccedApplyLabelsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyUnspeccedApplyLabelsRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.unspecced.applyLabels";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "labels" : self.params.labels }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}