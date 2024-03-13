use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_apply_writes`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoApplyWritesRequest {
    pub repo: String,
    pub swap_commit: Option<String>,
    pub validate: Option<bool>,
    pub writes: Vec<serde_json::Value>,
}
impl ComAtprotoRepoApplyWritesRequest {}
impl FluentRequest<'_, ComAtprotoRepoApplyWritesRequest> {
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
for FluentRequest<'a, ComAtprotoRepoApplyWritesRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.applyWrites";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "repo" : self.params.repo }));
            if let Some(ref unwrapped) = self.params.swap_commit {
                r = r.json(json!({ "swapCommit" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.validate {
                r = r.json(json!({ "validate" : unwrapped }));
            }
            r = r.json(json!({ "writes" : self.params.writes }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}