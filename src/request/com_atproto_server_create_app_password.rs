use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_create_app_password`].

On request success, this will return a [`ComAtprotoServerCreateAppPasswordAppPassword`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateAppPasswordRequest {
    pub name: String,
}
impl ComAtprotoServerCreateAppPasswordRequest {}
impl FluentRequest<'_, ComAtprotoServerCreateAppPasswordRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerCreateAppPasswordRequest> {
    type Output = httpclient::InMemoryResult<
        ComAtprotoServerCreateAppPasswordAppPassword,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.createAppPassword";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "name" : self.params.name }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}