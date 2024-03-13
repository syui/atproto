use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_sync_get_latest_commit`].

On request success, this will return a [`ComAtprotoSyncGetLatestCommitResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetLatestCommitRequest {
    pub did: String,
}
impl ComAtprotoSyncGetLatestCommitRequest {}
impl FluentRequest<'_, ComAtprotoSyncGetLatestCommitRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoSyncGetLatestCommitRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoSyncGetLatestCommitResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.sync.getLatestCommit";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}