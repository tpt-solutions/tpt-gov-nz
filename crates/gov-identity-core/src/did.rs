use crate::error::IdentityError;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

const DID_PREFIX: &str = "did:gov:nz:";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GovDid(String);

impl GovDid {
    pub fn from_verifying_key(key: &VerifyingKey) -> Self {
        use base64::Engine;
        let fingerprint =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(key.as_bytes());
        Self(format!("{DID_PREFIX}{fingerprint}"))
    }

    pub fn parse(s: &str) -> Result<Self, IdentityError> {
        if s.starts_with(DID_PREFIX) && s.len() > DID_PREFIX.len() {
            Ok(Self(s.to_owned()))
        } else {
            Err(IdentityError::InvalidDid(s.to_owned()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for GovDid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    pub did: GovDid,
    pub public_key_b64: String,
    pub created_at: i64,
}

impl DidDocument {
    pub fn new(signing_key: &SigningKey) -> Self {
        use base64::Engine;
        let verifying_key = signing_key.verifying_key();
        let did = GovDid::from_verifying_key(&verifying_key);
        let public_key_b64 =
            base64::engine::general_purpose::STANDARD.encode(verifying_key.as_bytes());
        Self {
            did,
            public_key_b64,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}
