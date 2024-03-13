use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsProfileAssociated {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedgens: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labeler: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lists: Option<i64>,
}
impl std::fmt::Display for AppBskyActorDefsProfileAssociated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}