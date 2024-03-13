use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_identity_update_handle`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentityUpdateHandleRequest {
    pub handle: String,
}
impl ComAtprotoIdentityUpdateHandleRequest {}
impl FluentRequest<'_, ComAtprotoIdentityUpdateHandleRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoIdentityUpdateHandleRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.identity.updateHandle";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "handle" : self.params.handle }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}