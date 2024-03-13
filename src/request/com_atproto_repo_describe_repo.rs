use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_describe_repo`].

On request success, this will return a [`ComAtprotoRepoDescribeRepoResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoDescribeRepoRequest {
    pub repo: String,
}
impl ComAtprotoRepoDescribeRepoRequest {}
impl FluentRequest<'_, ComAtprotoRepoDescribeRepoRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoRepoDescribeRepoRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoRepoDescribeRepoResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.describeRepo";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}