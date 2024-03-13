use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoSyncListReposRepo {
    pub did: String,
    ///Current repo commit CID
    pub head: String,
    pub rev: String,
}
impl std::fmt::Display for ComAtprotoSyncListReposRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}