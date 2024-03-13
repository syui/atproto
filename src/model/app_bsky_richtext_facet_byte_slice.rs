use serde::{Serialize, Deserialize};
///Specifies the sub-string range a facet feature applies to. Start index is inclusive, end index is exclusive. Indices are zero-indexed, counting bytes of the UTF-8 encoded text. NOTE: some languages, like Javascript, use UTF-16 or Unicode codepoints for string slice indexing; in these languages, convert to byte arrays before working with facets.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppBskyRichtextFacetByteSlice {
    #[serde(rename = "byteEnd")]
    pub byte_end: i64,
    #[serde(rename = "byteStart")]
    pub byte_start: i64,
}
impl std::fmt::Display for AppBskyRichtextFacetByteSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}