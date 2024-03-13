use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_temp_request_phone_verification`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoTempRequestPhoneVerificationRequest {
    pub phone_number: String,
}
impl ComAtprotoTempRequestPhoneVerificationRequest {}
impl FluentRequest<'_, ComAtprotoTempRequestPhoneVerificationRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoTempRequestPhoneVerificationRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.temp.requestPhoneVerification";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "phoneNumber" : self.params.phone_number }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}