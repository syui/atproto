use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneModerationDefsSubjectStatusView {
    ///True indicates that the a previously taken moderator action was appealed against, by the author of the content. False indicates last appeal was resolved by moderators.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appealed: Option<bool>,
    ///Sticky comment on the subject.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    ///Timestamp referencing the first moderation status impacting event was emitted on the subject
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: i64,
    ///Timestamp referencing when the author of the subject appealed a moderation action
    #[serde(rename = "lastAppealedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_appealed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "lastReportedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_reported_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "lastReviewedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "lastReviewedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_reviewed_by: Option<String>,
    #[serde(rename = "muteUntil")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute_until: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "reviewState")]
    pub review_state: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub subject: serde_json::Value,
    #[serde(rename = "subjectBlobCids")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_blob_cids: Option<Vec<String>>,
    #[serde(rename = "subjectRepoHandle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_repo_handle: Option<String>,
    #[serde(rename = "suspendUntil")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend_until: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub takendown: Option<bool>,
    ///Timestamp referencing when the last update was made to the moderation status of the subject
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for ToolsOzoneModerationDefsSubjectStatusView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}