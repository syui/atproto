use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_communication_delete_template`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneCommunicationDeleteTemplateRequest {
    pub id: String,
}
impl ToolsOzoneCommunicationDeleteTemplateRequest {}
impl FluentRequest<'_, ToolsOzoneCommunicationDeleteTemplateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneCommunicationDeleteTemplateRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.communication.deleteTemplate";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "id" : self.params.id }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}