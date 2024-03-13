use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyActorDefsPreferences(pub Vec<serde_json::Value>);