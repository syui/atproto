use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneCommunicationDefsTemplateView {
    ///Subject of the message, used in emails.
    #[serde(rename = "contentMarkdown")]
    pub content_markdown: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub disabled: bool,
    pub id: String,
    ///DID of the user who last updated the template.
    #[serde(rename = "lastUpdatedBy")]
    pub last_updated_by: String,
    ///Name of the template.
    pub name: String,
    ///Content of the template, can contain markdown and variable placeholders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for ToolsOzoneCommunicationDefsTemplateView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}