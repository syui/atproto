use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::app_bsky_labeler_get_services`].

On request success, this will return a [`AppBskyLabelerGetServicesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBskyLabelerGetServicesRequest {
    pub detailed: Option<bool>,
    pub dids: Vec<String>,
}
impl AppBskyLabelerGetServicesRequest {}
impl FluentRequest<'_, AppBskyLabelerGetServicesRequest> {
    ///Set the value of the detailed field.
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.params.detailed = Some(detailed);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AppBskyLabelerGetServicesRequest> {
    type Output = httpclient::InMemoryResult<AppBskyLabelerGetServicesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/app.bsky.labeler.getServices";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}