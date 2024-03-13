use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_request_email_update`].

On request success, this will return a [`ComAtprotoServerRequestEmailUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerRequestEmailUpdateRequest {}
impl ComAtprotoServerRequestEmailUpdateRequest {}
impl FluentRequest<'_, ComAtprotoServerRequestEmailUpdateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerRequestEmailUpdateRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerRequestEmailUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.requestEmailUpdate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}