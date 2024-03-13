use serde::{Serialize, Deserialize};
use super::AppBskyRichtextFacetByteSlice;
///Annotation of a sub-string within rich text.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacet {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<serde_json::Value>,
    ///Specifies the sub-string range a facet feature applies to. Start index is inclusive, end index is exclusive. Indices are zero-indexed, counting bytes of the UTF-8 encoded text. NOTE: some languages, like Javascript, use UTF-16 or Unicode codepoints for string slice indexing; in these languages, convert to byte arrays before working with facets.
    pub index: AppBskyRichtextFacetByteSlice,
}
impl std::fmt::Display for AppBskyRichtextFacet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}