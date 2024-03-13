use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_reset_password`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerResetPasswordRequest {
    pub password: String,
    pub token: String,
}
impl ComAtprotoServerResetPasswordRequest {}
impl FluentRequest<'_, ComAtprotoServerResetPasswordRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerResetPasswordRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.resetPassword";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "password" : self.params.password }));
            r = r.json(json!({ "token" : self.params.token }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}