use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComAtprotoServerReserveSigningKeyResponse {
    ///The public key for the reserved signing key, in did:key serialization.
    #[serde(rename = "signingKey")]
    pub signing_key: String,
}
impl std::fmt::Display for ComAtprotoServerReserveSigningKeyResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}