use serde::{Serialize, Deserialize};
///Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyFeedPostTextSlice {
    pub end: i64,
    pub start: i64,
}
impl std::fmt::Display for AppBskyFeedPostTextSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}