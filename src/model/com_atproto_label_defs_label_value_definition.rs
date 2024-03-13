use serde::{Serialize, Deserialize};
use super::ComAtprotoLabelDefsLabelValueDefinitionStrings;
///Declares a label value and its expected interpertations and behaviors.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelDefsLabelValueDefinition {
    ///Does the user need to have adult content enabled in order to configure this label?
    #[serde(rename = "adultOnly")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adult_only: Option<bool>,
    ///What should this label hide in the UI, if applied? 'content' hides all of the target; 'media' hides the images/video/audio; 'none' hides nothing.
    pub blurs: String,
    ///The default setting for this label.
    #[serde(rename = "defaultSetting")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_setting: Option<String>,
    ///The value of the label being defined. Must only include lowercase ascii and the '-' character ([a-z-]+).
    pub identifier: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locales: Vec<ComAtprotoLabelDefsLabelValueDefinitionStrings>,
    ///How should a client visually convey this label? 'inform' means neutral and informational; 'alert' means negative and warning; 'none' means show nothing.
    pub severity: String,
}
impl std::fmt::Display for ComAtprotoLabelDefsLabelValueDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}