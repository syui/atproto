use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_update_email`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerUpdateEmailRequest {
    pub email: String,
    pub token: Option<String>,
}
impl ComAtprotoServerUpdateEmailRequest {}
impl FluentRequest<'_, ComAtprotoServerUpdateEmailRequest> {
    ///Set the value of the token field.
    pub fn token(mut self, token: &str) -> Self {
        self.params.token = Some(token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerUpdateEmailRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.updateEmail";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "email" : self.params.email }));
            if let Some(ref unwrapped) = self.params.token {
                r = r.json(json!({ "token" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}