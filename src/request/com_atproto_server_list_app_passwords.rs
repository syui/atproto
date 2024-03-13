use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_list_app_passwords`].

On request success, this will return a [`ComAtprotoServerListAppPasswordsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerListAppPasswordsRequest {}
impl ComAtprotoServerListAppPasswordsRequest {}
impl FluentRequest<'_, ComAtprotoServerListAppPasswordsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerListAppPasswordsRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerListAppPasswordsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.listAppPasswords";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}