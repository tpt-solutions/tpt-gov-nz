use base64::Engine;
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
        Self {
            signing_key,
            verifying_key,
            dept_id: dept_id.into(),
        }
    }

    pub fn public_key_b64(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(self.verifying_key.as_bytes())
    }

    /// Get the base64-encoded secret key (private key).
    pub fn secret_key_b64(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(self.signing_key.to_bytes())
    }

    /// Create a NodeKeypair from a base64-encoded secret key.
    pub fn from_secret_b64(encoded: &str) -> anyhow::Result<Self> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded.trim())
            .map_err(|e| anyhow::anyhow!("invalid base64 secret key: {e}"))?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("secret key must be 32 bytes"))?;
        let signing_key = SigningKey::from_bytes(&arr);
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            signing_key,
            verifying_key,
            dept_id: "unknown".to_string(),
        })
    }

    /// Create a NodeKeypair from a base64-encoded secret key with explicit dept_id.
    pub fn from_secret_b64_with_dept_id(encoded: &str, dept_id: impl Into<String>) -> anyhow::Result<Self> {
        let mut keypair = Self::from_secret_b64(encoded)?;
        keypair.dept_id = dept_id.into();
        Ok(keypair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    #[test]
    fn from_secret_b64_round_trip() {
        let keypair = NodeKeypair::generate("test");
        let secret_b64 = keypair.secret_key_b64();
        
        let restored = NodeKeypair::from_secret_b64(&secret_b64).unwrap();
        assert_eq!(keypair.public_key_b64(), restored.public_key_b64());
    }

    #[test]
    fn from_secret_b64_with_dept_id() {
        let keypair = NodeKeypair::generate("test");
        let secret_b64 = keypair.secret_key_b64();
        
        let restored = NodeKeypair::from_secret_b64_with_dept_id(&secret_b64, "test-dept").unwrap();
        assert_eq!(keypair.public_key_b64(), restored.public_key_b64());
        assert_eq!(restored.dept_id, "test-dept");
    }

    #[test]
    fn from_secret_b64_rejects_bad_input() {
        assert!(NodeKeypair::from_secret_b64("not base64!").is_err());
        assert!(NodeKeypair::from_secret_b64(&base64::engine::general_purpose::STANDARD.encode([0u8; 31])).is_err());
    }
}
