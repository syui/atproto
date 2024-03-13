use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_server_create_account`].

On request success, this will return a [`ComAtprotoServerCreateAccountResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoServerCreateAccountRequest {
    pub did: Option<String>,
    pub email: Option<String>,
    pub handle: String,
    pub invite_code: Option<String>,
    pub password: Option<String>,
    pub plc_op: Option<serde_json::Value>,
    pub recovery_key: Option<String>,
    pub verification_code: Option<String>,
    pub verification_phone: Option<String>,
}
impl ComAtprotoServerCreateAccountRequest {}
impl FluentRequest<'_, ComAtprotoServerCreateAccountRequest> {
    ///Set the value of the did field.
    pub fn did(mut self, did: &str) -> Self {
        self.params.did = Some(did.to_owned());
        self
    }
    ///Set the value of the email field.
    pub fn email(mut self, email: &str) -> Self {
        self.params.email = Some(email.to_owned());
        self
    }
    ///Set the value of the invite_code field.
    pub fn invite_code(mut self, invite_code: &str) -> Self {
        self.params.invite_code = Some(invite_code.to_owned());
        self
    }
    ///Set the value of the password field.
    pub fn password(mut self, password: &str) -> Self {
        self.params.password = Some(password.to_owned());
        self
    }
    ///Set the value of the plc_op field.
    pub fn plc_op(mut self, plc_op: serde_json::Value) -> Self {
        self.params.plc_op = Some(plc_op);
        self
    }
    ///Set the value of the recovery_key field.
    pub fn recovery_key(mut self, recovery_key: &str) -> Self {
        self.params.recovery_key = Some(recovery_key.to_owned());
        self
    }
    ///Set the value of the verification_code field.
    pub fn verification_code(mut self, verification_code: &str) -> Self {
        self.params.verification_code = Some(verification_code.to_owned());
        self
    }
    ///Set the value of the verification_phone field.
    pub fn verification_phone(mut self, verification_phone: &str) -> Self {
        self.params.verification_phone = Some(verification_phone.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoServerCreateAccountRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoServerCreateAccountResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.server.createAccount";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.did {
                r = r.json(json!({ "did" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.email {
                r = r.json(json!({ "email" : unwrapped }));
            }
            r = r.json(json!({ "handle" : self.params.handle }));
            if let Some(ref unwrapped) = self.params.invite_code {
                r = r.json(json!({ "inviteCode" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.password {
                r = r.json(json!({ "password" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.plc_op {
                r = r.json(json!({ "plcOp" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.recovery_key {
                r = r.json(json!({ "recoveryKey" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.verification_code {
                r = r.json(json!({ "verificationCode" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.verification_phone {
                r = r.json(json!({ "verificationPhone" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}