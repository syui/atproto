use serde::{Serialize, Deserialize};
use super::ToolsOzoneCommunicationDefsTemplateView;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsOzoneCommunicationListTemplatesResponse {
    #[serde(rename = "communicationTemplates")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication_templates: Vec<ToolsOzoneCommunicationDefsTemplateView>,
}
impl std::fmt::Display for ToolsOzoneCommunicationListTemplatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}