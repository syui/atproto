use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_moderation_create_report`].

On request success, this will return a [`ComAtprotoModerationCreateReportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoModerationCreateReportRequest {
    pub reason: Option<String>,
    pub reason_type: ComAtprotoModerationDefsReasonType,
    pub subject: serde_json::Value,
}
impl ComAtprotoModerationCreateReportRequest {}
impl FluentRequest<'_, ComAtprotoModerationCreateReportRequest> {
    ///Set the value of the reason field.
    pub fn reason(mut self, reason: &str) -> Self {
        self.params.reason = Some(reason.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoModerationCreateReportRequest> {
    type Output = httpclient::InMemoryResult<ComAtprotoModerationCreateReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.moderation.createReport";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.reason {
                r = r.json(json!({ "reason" : unwrapped }));
            }
            r = r.json(json!({ "reasonType" : self.params.reason_type }));
            r = r.json(json!({ "subject" : self.params.subject }));
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}