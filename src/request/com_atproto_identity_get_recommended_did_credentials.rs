use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_identity_get_recommended_did_credentials`].

On request success, this will return a [`ComAtprotoIdentityGetRecommendedDidCredentialsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentityGetRecommendedDidCredentialsRequest {}
impl ComAtprotoIdentityGetRecommendedDidCredentialsRequest {}
impl FluentRequest<'_, ComAtprotoIdentityGetRecommendedDidCredentialsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoIdentityGetRecommendedDidCredentialsRequest> {
    type Output = httpclient::InMemoryResult<
        ComAtprotoIdentityGetRecommendedDidCredentialsResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.identity.getRecommendedDidCredentials";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}