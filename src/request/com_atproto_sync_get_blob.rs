use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_sync_get_blob`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetBlobRequest {
    pub cid: String,
    pub did: String,
}
impl ComAtprotoSyncGetBlobRequest {}
impl FluentRequest<'_, ComAtprotoSyncGetBlobRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ComAtprotoSyncGetBlobRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.sync.getBlob";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}