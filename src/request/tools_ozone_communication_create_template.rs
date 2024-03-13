use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_communication_create_template`].

On request success, this will return a [`ToolsOzoneCommunicationDefsTemplateView`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneCommunicationCreateTemplateRequest {
    pub content_markdown: String,
    pub created_by: Option<String>,
    pub name: String,
    pub subject: String,
}
impl ToolsOzoneCommunicationCreateTemplateRequest {}
impl FluentRequest<'_, ToolsOzoneCommunicationCreateTemplateRequest> {
    ///Set the value of the created_by field.
    pub fn created_by(mut self, created_by: &str) -> Self {
        self.params.created_by = Some(created_by.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneCommunicationCreateTemplateRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneCommunicationDefsTemplateView>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.communication.createTemplate";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "contentMarkdown" : self.params.content_markdown }));
            if let Some(ref unwrapped) = self.params.created_by {
                r = r.json(json!({ "createdBy" : unwrapped }));
            }
            r = r.json(json!({ "name" : self.params.name }));
            r = r.json(json!({ "subject" : self.params.subject }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}