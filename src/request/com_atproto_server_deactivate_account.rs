use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_deactivate_account`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerDeactivateAccountRequest {
    pub delete_after: Option<chrono::DateTime<chrono::Utc>>,
}
impl ComAtprotoServerDeactivateAccountRequest {}
impl FluentRequest<'_, ComAtprotoServerDeactivateAccountRequest> {
    ///Set the value of the delete_after field.
    pub fn delete_after(mut self, delete_after: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.delete_after = Some(delete_after);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerDeactivateAccountRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.deactivateAccount";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.delete_after {
                r = r.json(json!({ "deleteAfter" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}