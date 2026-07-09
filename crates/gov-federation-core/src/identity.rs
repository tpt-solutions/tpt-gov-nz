use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeptIdentity {
    pub dept_id: String,
    pub display_name: String,
    pub public_key_b64: String,
    pub endpoint: String,
}

pub struct NodeKeypair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub dept_id: String,
}

impl NodeKeypair {
    pub fn generate(dept_id: impl Into<String>) -> Self {
        use rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self { signing_key, verifying_key, dept_id: dept_id.into() }
    }

    pub fn public_key_b64(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(self.verifying_key.as_bytes())
    }
}
