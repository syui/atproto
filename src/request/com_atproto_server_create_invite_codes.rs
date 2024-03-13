use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_create_invite_codes`].

On request success, this will return a [`ComAtprotoServerCreateInviteCodesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateInviteCodesRequest {
    pub code_count: i64,
    pub for_accounts: Option<Vec<String>>,
    pub use_count: i64,
}
impl ComAtprotoServerCreateInviteCodesRequest {}
impl FluentRequest<'_, ComAtprotoServerCreateInviteCodesRequest> {
    ///Set the value of the for_accounts field.
    pub fn for_accounts(
        mut self,
        for_accounts: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .for_accounts = Some(
            for_accounts.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerCreateInviteCodesRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerCreateInviteCodesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.createInviteCodes";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "codeCount" : self.params.code_count }));
            if let Some(ref unwrapped) = self.params.for_accounts {
                r = r.json(json!({ "forAccounts" : unwrapped }));
            }
            r = r.json(json!({ "useCount" : self.params.use_count }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}