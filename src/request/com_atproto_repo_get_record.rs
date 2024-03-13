use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_get_record`].

On request success, this will return a [`ComAtprotoRepoGetRecordResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoGetRecordRequest {
    pub cid: Option<String>,
    pub collection: String,
    pub repo: String,
    pub rkey: String,
}
impl ComAtprotoRepoGetRecordRequest {}
impl FluentRequest<'_, ComAtprotoRepoGetRecordRequest> {
    ///Set the value of the cid field.
    pub fn cid(mut self, cid: &str) -> Self {
        self.params.cid = Some(cid.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoRepoGetRecordRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoRepoGetRecordResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.getRecord";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}