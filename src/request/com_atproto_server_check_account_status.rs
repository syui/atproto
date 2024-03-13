use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_check_account_status`].

On request success, this will return a [`ComAtprotoServerCheckAccountStatusResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCheckAccountStatusRequest {}
impl ComAtprotoServerCheckAccountStatusRequest {}
impl FluentRequest<'_, ComAtprotoServerCheckAccountStatusRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerCheckAccountStatusRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerCheckAccountStatusResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.checkAccountStatus";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}