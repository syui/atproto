use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_put_record`].

On request success, this will return a [`ComAtprotoRepoPutRecordResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoPutRecordRequest {
    pub collection: String,
    pub record: serde_json::Value,
    pub repo: String,
    pub rkey: String,
    pub swap_commit: Option<String>,
    pub swap_record: Option<String>,
    pub validate: Option<bool>,
}
impl ComAtprotoRepoPutRecordRequest {}
pub struct ComAtprotoRepoPutRecordRequired<'a> {
    pub collection: &'a str,
    pub record: serde_json::Value,
    pub repo: &'a str,
    pub rkey: &'a str,
}
impl<'a> ComAtprotoRepoPutRecordRequired<'a> {}
impl FluentRequest<'_, ComAtprotoRepoPutRecordRequest> {
    ///Set the value of the swap_commit field.
    pub fn swap_commit(mut self, swap_commit: &str) -> Self {
        self.params.swap_commit = Some(swap_commit.to_owned());
        self
    }
    ///Set the value of the swap_record field.
    pub fn swap_record(mut self, swap_record: &str) -> Self {
        self.params.swap_record = Some(swap_record.to_owned());
        self
    }
    ///Set the value of the validate field.
    pub fn validate(mut self, validate: bool) -> Self {
        self.params.validate = Some(validate);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoRepoPutRecordRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoRepoPutRecordResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.putRecord";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "collection" : self.params.collection }));
            r = r.json(json!({ "record" : self.params.record }));
            r = r.json(json!({ "repo" : self.params.repo }));
            r = r.json(json!({ "rkey" : self.params.rkey }));
            if let Some(ref unwrapped) = self.params.swap_commit {
                r = r.json(json!({ "swapCommit" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.swap_record {
                r = r.json(json!({ "swapRecord" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.validate {
                r = r.json(json!({ "validate" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}