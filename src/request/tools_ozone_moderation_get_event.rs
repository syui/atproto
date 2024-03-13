use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_moderation_get_event`].

On request success, this will return a [`ToolsOzoneModerationDefsModEventViewDetail`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneModerationGetEventRequest {
    pub id: i64,
}
impl ToolsOzoneModerationGetEventRequest {}
impl FluentRequest<'_, ToolsOzoneModerationGetEventRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneModerationGetEventRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneModerationDefsModEventViewDetail>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.moderation.getEvent";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}