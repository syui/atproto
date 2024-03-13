use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_moderation_get_record`].

On request success, this will return a [`ToolsOzoneModerationDefsRecordViewDetail`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneModerationGetRecordRequest {
    pub cid: Option<String>,
    pub uri: String,
}
impl ToolsOzoneModerationGetRecordRequest {}
impl FluentRequest<'_, ToolsOzoneModerationGetRecordRequest> {
    ///Set the value of the cid field.
    pub fn cid(mut self, cid: &str) -> Self {
        self.params.cid = Some(cid.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneModerationGetRecordRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneModerationDefsRecordViewDetail>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.moderation.getRecord";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}