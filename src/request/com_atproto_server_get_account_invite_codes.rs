use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_get_account_invite_codes`].

On request success, this will return a [`ComAtprotoServerGetAccountInviteCodesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerGetAccountInviteCodesRequest {
    pub create_available: Option<bool>,
    pub include_used: Option<bool>,
}
impl ComAtprotoServerGetAccountInviteCodesRequest {}
impl FluentRequest<'_, ComAtprotoServerGetAccountInviteCodesRequest> {
    ///Set the value of the create_available field.
    pub fn create_available(mut self, create_available: bool) -> Self {
        self.params.create_available = Some(create_available);
        self
    }
    ///Set the value of the include_used field.
    pub fn include_used(mut self, include_used: bool) -> Self {
        self.params.include_used = Some(include_used);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerGetAccountInviteCodesRequest> {
    type Output = httpclient::InMemoryResult<
        ComAtprotoServerGetAccountInviteCodesResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.getAccountInviteCodes";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}