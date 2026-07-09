use crate::error::FederationError;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Canonical JSON payload signed by the auditor (excludes `signature`).
#[derive(Serialize)]
struct AuditSignPayload<'a> {
    id: &'a Uuid,
    message_id: &'a Uuid,
    citizen_did: &'a str,
    action: &'a str,
    from_dept_id: &'a str,
    to_dept_id: &'a str,
    scopes_accessed: &'a [String],
    timestamp_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub message_id: Uuid,
    pub citizen_did: String,
    pub action: String,
    pub from_dept_id: String,
    pub to_dept_id: String,
    pub scopes_accessed: Vec<String>,
    pub timestamp_ms: i64,
    pub signature: String,
}

impl AuditLogEntry {
    pub fn new(
        message_id: Uuid,
        citizen_did: impl Into<String>,
        action: impl Into<String>,
        from_dept_id: impl Into<String>,
        to_dept_id: impl Into<String>,
        scopes_accessed: Vec<String>,
        signature: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            message_id,
            citizen_did: citizen_did.into(),
            action: action.into(),
            from_dept_id: from_dept_id.into(),
            to_dept_id: to_dept_id.into(),
            scopes_accessed,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            signature,
        }
    }

    fn payload_hash(&self) -> [u8; 32] {
        let payload = AuditSignPayload {
            id: &self.id,
            message_id: &self.message_id,
            citizen_did: &self.citizen_did,
            action: &self.action,
            from_dept_id: &self.from_dept_id,
            to_dept_id: &self.to_dept_id,
            scopes_accessed: &self.scopes_accessed,
            timestamp_ms: self.timestamp_ms,
        };
        let json = serde_json::to_vec(&payload).expect("serialisation must not fail");
        Sha256::digest(&json).into()
    }

    /// Sign this audit entry with the auditor's Ed25519 keypair.
    pub fn sign(&mut self, signing_key: &SigningKey) {
        use base64::Engine;
        let hash = self.payload_hash();
        let sig = signing_key.sign(&hash);
        self.signature = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());
    }

    /// Verify the audit entry's signature against the auditor's public key.
    pub fn verify(&self, auditor_key: &VerifyingKey) -> Result<(), FederationError> {
        use base64::Engine;
        let hash = self.payload_hash();
        let sig_bytes = base64::engine::general_purpose::STANDARD
            .decode(&self.signature)
            .map_err(|e| FederationError::Crypto(format!("invalid base64 signature: {e}")))?;
        let sig = ed25519_dalek::Signature::from_bytes(
            &sig_bytes
                .try_into()
                .map_err(|_| FederationError::Crypto("signature length invalid".into()))?,
        );
        auditor_key
            .verify(&hash, &sig)
            .map_err(|_| FederationError::SignatureInvalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_round_trip() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let mut entry = AuditLogEntry::new(
            Uuid::new_v4(),
            "did:gov:nz:citizen001",
            "DATA_REQUEST",
            "winz",
            "ird",
            vec!["ird:income".into()],
            String::new(),
        );
        entry.sign(&signing_key);

        assert!(!entry.signature.is_empty());
        assert!(entry.verify(&verifying_key).is_ok());
    }

    #[test]
    fn verify_fails_with_wrong_key() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let wrong_key = SigningKey::generate(&mut OsRng);

        let mut entry = AuditLogEntry::new(
            Uuid::new_v4(),
            "did:gov:nz:citizen002",
            "DATA_RESPONSE",
            "ird",
            "winz",
            vec!["ird:tax-summary".into()],
            String::new(),
        );
        entry.sign(&signing_key);

        assert!(entry.verify(&wrong_key.verifying_key()).is_err());
    }
}
