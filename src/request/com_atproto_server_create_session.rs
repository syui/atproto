use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_create_session`].

On request success, this will return a [`ComAtprotoServerCreateSessionResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateSessionRequest {
    pub identifier: String,
    pub password: String,
}
impl ComAtprotoServerCreateSessionRequest {}
impl FluentRequest<'_, ComAtprotoServerCreateSessionRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerCreateSessionRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerCreateSessionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.createSession";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "identifier" : self.params.identifier }));
            r = r.json(json!({ "password" : self.params.password }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}