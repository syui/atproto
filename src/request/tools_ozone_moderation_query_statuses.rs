use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::tools_ozone_moderation_query_statuses`].

On request success, this will return a [`ToolsOzoneModerationQueryStatusesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsOzoneModerationQueryStatusesRequest {
    pub appealed: Option<bool>,
    pub comment: Option<String>,
    pub cursor: Option<String>,
    pub exclude_tags: Option<Vec<String>>,
    pub ignore_subjects: Option<Vec<String>>,
    pub include_muted: Option<bool>,
    pub last_reviewed_by: Option<String>,
    pub limit: Option<i64>,
    pub reported_after: Option<chrono::DateTime<chrono::Utc>>,
    pub reported_before: Option<chrono::DateTime<chrono::Utc>>,
    pub review_state: Option<String>,
    pub reviewed_after: Option<chrono::DateTime<chrono::Utc>>,
    pub reviewed_before: Option<chrono::DateTime<chrono::Utc>>,
    pub sort_direction: Option<String>,
    pub sort_field: Option<String>,
    pub subject: Option<String>,
    pub tags: Option<Vec<String>>,
    pub takendown: Option<bool>,
}
impl ToolsOzoneModerationQueryStatusesRequest {}
impl FluentRequest<'_, ToolsOzoneModerationQueryStatusesRequest> {
    ///Set the value of the appealed field.
    pub fn appealed(mut self, appealed: bool) -> Self {
        self.params.appealed = Some(appealed);
        self
    }
    ///Set the value of the comment field.
    pub fn comment(mut self, comment: &str) -> Self {
        self.params.comment = Some(comment.to_owned());
        self
    }
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the exclude_tags field.
    pub fn exclude_tags(
        mut self,
        exclude_tags: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .exclude_tags = Some(
            exclude_tags.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the ignore_subjects field.
    pub fn ignore_subjects(
        mut self,
        ignore_subjects: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .ignore_subjects = Some(
            ignore_subjects.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the include_muted field.
    pub fn include_muted(mut self, include_muted: bool) -> Self {
        self.params.include_muted = Some(include_muted);
        self
    }
    ///Set the value of the last_reviewed_by field.
    pub fn last_reviewed_by(mut self, last_reviewed_by: &str) -> Self {
        self.params.last_reviewed_by = Some(last_reviewed_by.to_owned());
        self
    }
    ///Set the value of the limit field.
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    ///Set the value of the reported_after field.
    pub fn reported_after(
        mut self,
        reported_after: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.reported_after = Some(reported_after);
        self
    }
    ///Set the value of the reported_before field.
    pub fn reported_before(
        mut self,
        reported_before: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.reported_before = Some(reported_before);
        self
    }
    ///Set the value of the review_state field.
    pub fn review_state(mut self, review_state: &str) -> Self {
        self.params.review_state = Some(review_state.to_owned());
        self
    }
    ///Set the value of the reviewed_after field.
    pub fn reviewed_after(
        mut self,
        reviewed_after: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.reviewed_after = Some(reviewed_after);
        self
    }
    ///Set the value of the reviewed_before field.
    pub fn reviewed_before(
        mut self,
        reviewed_before: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.reviewed_before = Some(reviewed_before);
        self
    }
    ///Set the value of the sort_direction field.
    pub fn sort_direction(mut self, sort_direction: &str) -> Self {
        self.params.sort_direction = Some(sort_direction.to_owned());
        self
    }
    ///Set the value of the sort_field field.
    pub fn sort_field(mut self, sort_field: &str) -> Self {
        self.params.sort_field = Some(sort_field.to_owned());
        self
    }
    ///Set the value of the subject field.
    pub fn subject(mut self, subject: &str) -> Self {
        self.params.subject = Some(subject.to_owned());
        self
    }
    ///Set the value of the tags field.
    pub fn tags(mut self, tags: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .tags = Some(tags.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    ///Set the value of the takendown field.
    pub fn takendown(mut self, takendown: bool) -> Self {
        self.params.takendown = Some(takendown);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ToolsOzoneModerationQueryStatusesRequest> {
    type Output = httpclient::InMemoryResult<ToolsOzoneModerationQueryStatusesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tools.ozone.moderation.queryStatuses";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}