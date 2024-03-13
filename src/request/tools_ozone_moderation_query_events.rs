use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_moderation_query_events`].

On request success, this will return a [`ToolsOzoneModerationQueryEventsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneModerationQueryEventsRequest {
    pub added_labels: Option<Vec<String>>,
    pub added_tags: Option<Vec<String>>,
    pub comment: Option<String>,
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    pub created_by: Option<String>,
    pub cursor: Option<String>,
    pub has_comment: Option<bool>,
    pub include_all_user_records: Option<bool>,
    pub limit: Option<i64>,
    pub removed_labels: Option<Vec<String>>,
    pub removed_tags: Option<Vec<String>>,
    pub report_types: Option<Vec<String>>,
    pub sort_direction: Option<String>,
    pub subject: Option<String>,
    pub types: Option<Vec<String>>,
}
impl ToolsOzoneModerationQueryEventsRequest {}
impl FluentRequest<'_, ToolsOzoneModerationQueryEventsRequest> {
    ///Set the value of the added_labels field.
    pub fn added_labels(
        mut self,
        added_labels: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .added_labels = Some(
            added_labels.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the added_tags field.
    pub fn added_tags(
        mut self,
        added_tags: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .added_tags = Some(
            added_tags.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the comment field.
    pub fn comment(mut self, comment: &str) -> Self {
        self.params.comment = Some(comment.to_owned());
        self
    }
    ///Set the value of the created_after field.
    pub fn created_after(
        mut self,
        created_after: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.created_after = Some(created_after);
        self
    }
    ///Set the value of the created_before field.
    pub fn created_before(
        mut self,
        created_before: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.created_before = Some(created_before);
        self
    }
    ///Set the value of the created_by field.
    pub fn created_by(mut self, created_by: &str) -> Self {
        self.params.created_by = Some(created_by.to_owned());
        self
    }
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the has_comment field.
    pub fn has_comment(mut self, has_comment: bool) -> Self {
        self.params.has_comment = Some(has_comment);
        self
    }
    ///Set the value of the include_all_user_records field.
    pub fn include_all_user_records(mut self, include_all_user_records: bool) -> Self {
        self.params.include_all_user_records = Some(include_all_user_records);
        self
    }
    ///Set the value of the limit field.
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    ///Set the value of the removed_labels field.
    pub fn removed_labels(
        mut self,
        removed_labels: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .removed_labels = Some(
            removed_labels.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the removed_tags field.
    pub fn removed_tags(
        mut self,
        removed_tags: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .removed_tags = Some(
            removed_tags.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the report_types field.
    pub fn report_types(
        mut self,
        report_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .report_types = Some(
            report_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the sort_direction field.
    pub fn sort_direction(mut self, sort_direction: &str) -> Self {
        self.params.sort_direction = Some(sort_direction.to_owned());
        self
    }
    ///Set the value of the subject field.
    pub fn subject(mut self, subject: &str) -> Self {
        self.params.subject = Some(subject.to_owned());
        self
    }
    ///Set the value of the types field.
    pub fn types(mut self, types: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .types = Some(types.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneModerationQueryEventsRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneModerationQueryEventsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.moderation.queryEvents";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}