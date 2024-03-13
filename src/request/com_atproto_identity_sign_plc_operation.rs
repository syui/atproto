use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_identity_sign_plc_operation`].

On request success, this will return a [`ComAtprotoIdentitySignPlcOperationResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoIdentitySignPlcOperationRequest {
    pub also_known_as: Option<Vec<String>>,
    pub rotation_keys: Option<Vec<String>>,
    pub services: Option<serde_json::Value>,
    pub token: Option<String>,
    pub verification_methods: Option<serde_json::Value>,
}
impl ComAtprotoIdentitySignPlcOperationRequest {}
impl FluentRequest<'_, ComAtprotoIdentitySignPlcOperationRequest> {
    ///Set the value of the also_known_as field.
    pub fn also_known_as(
        mut self,
        also_known_as: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .also_known_as = Some(
            also_known_as.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the rotation_keys field.
    pub fn rotation_keys(
        mut self,
        rotation_keys: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .rotation_keys = Some(
            rotation_keys.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the services field.
    pub fn services(mut self, services: serde_json::Value) -> Self {
        self.params.services = Some(services);
        self
    }
    ///Set the value of the token field.
    pub fn token(mut self, token: &str) -> Self {
        self.params.token = Some(token.to_owned());
        self
    }
    ///Set the value of the verification_methods field.
    pub fn verification_methods(
        mut self,
        verification_methods: serde_json::Value,
    ) -> Self {
        self.params.verification_methods = Some(verification_methods);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoIdentitySignPlcOperationRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoIdentitySignPlcOperationResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.identity.signPlcOperation";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.also_known_as {
                r = r.json(json!({ "alsoKnownAs" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.rotation_keys {
                r = r.json(json!({ "rotationKeys" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.services {
                r = r.json(json!({ "services" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.token {
                r = r.json(json!({ "token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.verification_methods {
                r = r.json(json!({ "verificationMethods" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}