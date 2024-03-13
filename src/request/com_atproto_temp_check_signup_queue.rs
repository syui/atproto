use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_temp_check_signup_queue`].

On request success, this will return a [`ComAtprotoTempCheckSignupQueueResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoTempCheckSignupQueueRequest {}
impl ComAtprotoTempCheckSignupQueueRequest {}
impl FluentRequest<'_, ComAtprotoTempCheckSignupQueueRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoTempCheckSignupQueueRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoTempCheckSignupQueueResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.temp.checkSignupQueue";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}