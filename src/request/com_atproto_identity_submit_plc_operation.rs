use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_identity_submit_plc_operation`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentitySubmitPlcOperationRequest {
    pub operation: serde_json::Value,
}
impl ComAtprotoIdentitySubmitPlcOperationRequest {}
impl FluentRequest<'_, ComAtprotoIdentitySubmitPlcOperationRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoIdentitySubmitPlcOperationRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.identity.submitPlcOperation";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "operation" : self.params.operation }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}