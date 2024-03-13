use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_confirm_email`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerConfirmEmailRequest {
    pub email: String,
    pub token: String,
}
impl ComAtprotoServerConfirmEmailRequest {}
impl FluentRequest<'_, ComAtprotoServerConfirmEmailRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerConfirmEmailRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.confirmEmail";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "email" : self.params.email }));
            r = r.json(json!({ "token" : self.params.token }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}