use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_communication_update_template`].

On request success, this will return a [`ToolsOzoneCommunicationDefsTemplateView`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneCommunicationUpdateTemplateRequest {
    pub content_markdown: Option<String>,
    pub disabled: Option<bool>,
    pub id: String,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub updated_by: Option<String>,
}
impl ToolsOzoneCommunicationUpdateTemplateRequest {}
impl FluentRequest<'_, ToolsOzoneCommunicationUpdateTemplateRequest> {
    ///Set the value of the content_markdown field.
    pub fn content_markdown(mut self, content_markdown: &str) -> Self {
        self.params.content_markdown = Some(content_markdown.to_owned());
        self
    }
    ///Set the value of the disabled field.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.params.disabled = Some(disabled);
        self
    }
    ///Set the value of the name field.
    pub fn name(mut self, name: &str) -> Self {
        self.params.name = Some(name.to_owned());
        self
    }
    ///Set the value of the subject field.
    pub fn subject(mut self, subject: &str) -> Self {
        self.params.subject = Some(subject.to_owned());
        self
    }
    ///Set the value of the updated_by field.
    pub fn updated_by(mut self, updated_by: &str) -> Self {
        self.params.updated_by = Some(updated_by.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneCommunicationUpdateTemplateRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneCommunicationDefsTemplateView>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.communication.updateTemplate";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.content_markdown {
                r = r.json(json!({ "contentMarkdown" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.disabled {
                r = r.json(json!({ "disabled" : unwrapped }));
            }
            r = r.json(json!({ "id" : self.params.id }));
            if let Some(ref unwrapped) = self.params.name {
                r = r.json(json!({ "name" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.subject {
                r = r.json(json!({ "subject" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.updated_by {
                r = r.json(json!({ "updatedBy" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}