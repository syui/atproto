use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_create_invite_code`].

On request success, this will return a [`ComAtprotoServerCreateInviteCodeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateInviteCodeRequest {
    pub for_account: Option<String>,
    pub use_count: i64,
}
impl ComAtprotoServerCreateInviteCodeRequest {}
impl FluentRequest<'_, ComAtprotoServerCreateInviteCodeRequest> {
    ///Set the value of the for_account field.
    pub fn for_account(mut self, for_account: &str) -> Self {
        self.params.for_account = Some(for_account.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerCreateInviteCodeRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerCreateInviteCodeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.createInviteCode";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.for_account {
                r = r.json(json!({ "forAccount" : unwrapped }));
            }
            r = r.json(json!({ "useCount" : self.params.use_count }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}