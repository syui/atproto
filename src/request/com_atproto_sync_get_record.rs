use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_sync_get_record`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetRecordRequest {
    pub collection: String,
    pub commit: Option<String>,
    pub did: String,
    pub rkey: String,
}
impl ComAtprotoSyncGetRecordRequest {}
impl FluentRequest<'_, ComAtprotoSyncGetRecordRequest> {
    ///Set the value of the commit field.
    pub fn commit(mut self, commit: &str) -> Self {
        self.params.commit = Some(commit.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoSyncGetRecordRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.sync.getRecord";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}