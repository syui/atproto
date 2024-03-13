use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_sync_get_head`].

On request success, this will return a [`ComAtprotoSyncGetHeadResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoSyncGetHeadRequest {
    pub did: String,
}
impl ComAtprotoSyncGetHeadRequest {}
impl FluentRequest<'_, ComAtprotoSyncGetHeadRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ComAtprotoSyncGetHeadRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoSyncGetHeadResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.sync.getHead";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}