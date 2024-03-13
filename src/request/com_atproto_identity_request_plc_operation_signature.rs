use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_identity_request_plc_operation_signature`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentityRequestPlcOperationSignatureRequest {}
impl ComAtprotoIdentityRequestPlcOperationSignatureRequest {}
impl FluentRequest<'_, ComAtprotoIdentityRequestPlcOperationSignatureRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoIdentityRequestPlcOperationSignatureRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.identity.requestPlcOperationSignature";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}