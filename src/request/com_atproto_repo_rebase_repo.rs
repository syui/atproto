use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_rebase_repo`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoRebaseRepoRequest {
    pub repo: String,
    pub swap_commit: Option<String>,
}
impl ComAtprotoRepoRebaseRepoRequest {}
impl FluentRequest<'_, ComAtprotoRepoRebaseRepoRequest> {
    ///Set the value of the swap_commit field.
    pub fn swap_commit(mut self, swap_commit: &str) -> Self {
        self.params.swap_commit = Some(swap_commit.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoRepoRebaseRepoRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.rebaseRepo";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "repo" : self.params.repo }));
            if let Some(ref unwrapped) = self.params.swap_commit {
                r = r.json(json!({ "swapCommit" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}