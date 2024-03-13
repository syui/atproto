use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_sync_get_repo`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetRepoRequest {
    pub did: String,
    pub since: Option<String>,
}
impl ComAtprotoSyncGetRepoRequest {}
impl FluentRequest<'_, ComAtprotoSyncGetRepoRequest> {
    ///Set the value of the since field.
    pub fn since(mut self, since: &str) -> Self {
        self.params.since = Some(since.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ComAtprotoSyncGetRepoRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.sync.getRepo";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}