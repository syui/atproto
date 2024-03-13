use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_moderation_emit_event`].

On request success, this will return a [`ToolsOzoneModerationDefsModEventView`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneModerationEmitEventRequest {
    pub created_by: String,
    pub event: serde_json::Value,
    pub subject: serde_json::Value,
    pub subject_blob_cids: Option<Vec<String>>,
}
impl ToolsOzoneModerationEmitEventRequest {}
impl FluentRequest<'_, ToolsOzoneModerationEmitEventRequest> {
    ///Set the value of the subject_blob_cids field.
    pub fn subject_blob_cids(
        mut self,
        subject_blob_cids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .subject_blob_cids = Some(
            subject_blob_cids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneModerationEmitEventRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneModerationDefsModEventView>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.moderation.emitEvent";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "createdBy" : self.params.created_by }));
            r = r.json(json!({ "event" : self.params.event }));
            r = r.json(json!({ "subject" : self.params.subject }));
            if let Some(ref unwrapped) = self.params.subject_blob_cids {
                r = r.json(json!({ "subjectBlobCids" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}