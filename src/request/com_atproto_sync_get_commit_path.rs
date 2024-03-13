use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_sync_get_commit_path`].

On request success, this will return a [`ComAtprotoSyncGetCommitPathResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetCommitPathRequest {
    pub did: String,
    pub earliest: Option<String>,
    pub latest: Option<String>,
}
impl ComAtprotoSyncGetCommitPathRequest {}
impl FluentRequest<'_, ComAtprotoSyncGetCommitPathRequest> {
    ///Set the value of the earliest field.
    pub fn earliest(mut self, earliest: &str) -> Self {
        self.params.earliest = Some(earliest.to_owned());
        self
    }
    ///Set the value of the latest field.
    pub fn latest(mut self, latest: &str) -> Self {
        self.params.latest = Some(latest.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoSyncGetCommitPathRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoSyncGetCommitPathResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.sync.getCommitPath";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}