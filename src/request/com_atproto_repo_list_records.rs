use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_list_records`].

On request success, this will return a [`ComAtprotoRepoListRecordsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoListRecordsRequest {
    pub collection: String,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub repo: String,
    pub reverse: Option<bool>,
    pub rkey_end: Option<String>,
    pub rkey_start: Option<String>,
}
impl ComAtprotoRepoListRecordsRequest {}
impl FluentRequest<'_, ComAtprotoRepoListRecordsRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the limit field.
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    ///Set the value of the reverse field.
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.params.reverse = Some(reverse);
        self
    }
    ///Set the value of the rkey_end field.
    pub fn rkey_end(mut self, rkey_end: &str) -> Self {
        self.params.rkey_end = Some(rkey_end.to_owned());
        self
    }
    ///Set the value of the rkey_start field.
    pub fn rkey_start(mut self, rkey_start: &str) -> Self {
        self.params.rkey_start = Some(rkey_start.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoRepoListRecordsRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoRepoListRecordsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.listRecords";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}