use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_request_password_reset`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerRequestPasswordResetRequest {
    pub email: String,
}
impl ComAtprotoServerRequestPasswordResetRequest {}
impl FluentRequest<'_, ComAtprotoServerRequestPasswordResetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerRequestPasswordResetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.requestPasswordReset";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "email" : self.params.email }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}