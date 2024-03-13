use serde::{Serialize, Deserialize};
///width:height represents an aspect ratio. It may be approximate, and may not correspond to absolute dimensions in any given unit.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyEmbedImagesAspectRatio {
    pub height: i64,
    pub width: i64,
}
impl std::fmt::Display for AppBskyEmbedImagesAspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}