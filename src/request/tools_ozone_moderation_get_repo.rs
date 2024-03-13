use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_moderation_get_repo`].

On request success, this will return a [`ToolsOzoneModerationDefsRepoViewDetail`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneModerationGetRepoRequest {
    pub did: String,
}
impl ToolsOzoneModerationGetRepoRequest {}
impl FluentRequest<'_, ToolsOzoneModerationGetRepoRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneModerationGetRepoRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneModerationDefsRepoViewDetail>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.moderation.getRepo";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}