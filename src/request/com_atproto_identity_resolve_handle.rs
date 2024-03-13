use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_identity_resolve_handle`].

On request success, this will return a [`ComAtprotoIdentityResolveHandleResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentityResolveHandleRequest {
    pub handle: String,
}
impl ComAtprotoIdentityResolveHandleRequest {}
impl FluentRequest<'_, ComAtprotoIdentityResolveHandleRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoIdentityResolveHandleRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoIdentityResolveHandleResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.identity.resolveHandle";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}