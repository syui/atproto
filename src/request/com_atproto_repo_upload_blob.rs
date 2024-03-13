use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_repo_upload_blob`].

On request success, this will return a [`ComAtprotoRepoUploadBlobResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoRepoUploadBlobRequest {}
impl ComAtprotoRepoUploadBlobRequest {}
impl FluentRequest<'_, ComAtprotoRepoUploadBlobRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoRepoUploadBlobRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoRepoUploadBlobResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.repo.uploadBlob";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}