use serde::{Serialize, Deserialize};
///Metadata tag on an atproto resource (eg, repo or record).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoLabelDefsLabel {
    ///Optionally, CID specifying the specific version of 'uri' resource this label applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    ///Timestamp when this label was created.
    pub cts: chrono::DateTime<chrono::Utc>,
    ///Timestamp at which this label expires (no longer applies).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<chrono::DateTime<chrono::Utc>>,
    ///If true, this is a negation label, overwriting a previous label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neg: Option<bool>,
    ///Signature of dag-cbor encoded label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    ///DID of the actor who created this label.
    pub src: String,
    ///AT URI of the record, repository (account), or other resource that this label applies to.
    pub uri: String,
    ///The short string name of the value or type of this label.
    pub val: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ver: Option<i64>,
}
impl std::fmt::Display for ComAtprotoLabelDefsLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}