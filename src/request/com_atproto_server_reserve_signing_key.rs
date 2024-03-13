use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_reserve_signing_key`].

On request success, this will return a [`ComAtprotoServerReserveSigningKeyResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerReserveSigningKeyRequest {
    pub did: Option<String>,
}
impl ComAtprotoServerReserveSigningKeyRequest {}
impl FluentRequest<'_, ComAtprotoServerReserveSigningKeyRequest> {
    ///Set the value of the did field.
    pub fn did(mut self, did: &str) -> Self {
        self.params.did = Some(did.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerReserveSigningKeyRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerReserveSigningKeyResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.reserveSigningKey";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.did {
                r = r.json(json!({ "did" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}