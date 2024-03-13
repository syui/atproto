use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyGraphDefsListPurpose(pub serde_json::Value);