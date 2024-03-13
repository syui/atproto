use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_revoke_app_password`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerRevokeAppPasswordRequest {
    pub name: String,
}
impl ComAtprotoServerRevokeAppPasswordRequest {}
impl FluentRequest<'_, ComAtprotoServerRevokeAppPasswordRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerRevokeAppPasswordRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.revokeAppPassword";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "name" : self.params.name }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}