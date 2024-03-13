use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_get_service_auth`].

On request success, this will return a [`ComAtprotoServerGetServiceAuthResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerGetServiceAuthRequest {
    pub aud: String,
}
impl ComAtprotoServerGetServiceAuthRequest {}
impl FluentRequest<'_, ComAtprotoServerGetServiceAuthRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerGetServiceAuthRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerGetServiceAuthResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.getServiceAuth";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}