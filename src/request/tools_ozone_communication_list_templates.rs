use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_communication_list_templates`].

On request success, this will return a [`ToolsOzoneCommunicationListTemplatesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneCommunicationListTemplatesRequest {}
impl ToolsOzoneCommunicationListTemplatesRequest {}
impl FluentRequest<'_, ToolsOzoneCommunicationListTemplatesRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneCommunicationListTemplatesRequest> {
    type Output = httpclient::InMemoryResult<
        ToolsOzoneCommunicationListTemplatesResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.communication.listTemplates";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}