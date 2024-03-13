use serde::{Serialize, Deserialize};
///Strings which describe the label in the UI, localized into a specific language.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelDefsLabelValueDefinitionStrings {
    ///A longer description of what the label means and why it might be applied.
    pub description: String,
    ///The code of the language these strings are written in.
    pub lang: String,
    ///A short human-readable name for the label.
    pub name: String,
}
impl std::fmt::Display for ComAtprotoLabelDefsLabelValueDefinitionStrings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}