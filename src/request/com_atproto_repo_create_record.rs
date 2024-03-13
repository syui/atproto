use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_create_record`].

On request success, this will return a [`ComAtprotoRepoCreateRecordResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoCreateRecordRequest {
    pub collection: String,
    pub record: serde_json::Value,
    pub repo: String,
    pub rkey: Option<String>,
    pub swap_commit: Option<String>,
    pub validate: Option<bool>,
}
impl ComAtprotoRepoCreateRecordRequest {}
impl FluentRequest<'_, ComAtprotoRepoCreateRecordRequest> {
    ///Set the value of the rkey field.
    pub fn rkey(mut self, rkey: &str) -> Self {
        self.params.rkey = Some(rkey.to_owned());
        self
    }
    ///Set the value of the swap_commit field.
    pub fn swap_commit(mut self, swap_commit: &str) -> Self {
        self.params.swap_commit = Some(swap_commit.to_owned());
        self
    }
    ///Set the value of the validate field.
    pub fn validate(mut self, validate: bool) -> Self {
        self.params.validate = Some(validate);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoRepoCreateRecordRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoRepoCreateRecordResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.createRecord";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "collection" : self.params.collection }));
            r = r.json(json!({ "record" : self.params.record }));
            r = r.json(json!({ "repo" : self.params.repo }));
            if let Some(ref unwrapped) = self.params.rkey {
                r = r.json(json!({ "rkey" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.swap_commit {
                r = r.json(json!({ "swapCommit" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.validate {
                r = r.json(json!({ "validate" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}