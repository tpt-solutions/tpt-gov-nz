use crate::error::FederationError;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Canonical JSON payload signed by the sender (excludes `signature`).
#[derive(Serialize)]
struct EnvelopeSignPayload<'a> {
    message_id: &'a Uuid,
    correlation_id: Option<Uuid>,
    message_type: &'a FederationMessageType,
    from_dept_id: &'a str,
    to_dept_id: &'a str,
    timestamp_ms: i64,
    consent_grant_ids: &'a [Uuid],
    payload_encrypted: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederationEnvelope {
    pub message_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    pub message_type: FederationMessageType,
    pub from_dept_id: String,
    pub to_dept_id: String,
    pub timestamp_ms: i64,
    pub consent_grant_ids: Vec<Uuid>,
    pub payload_encrypted: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FederationMessageType {
    DataRequest,
    DataResponse,
    DataDenied,
    AuditAck,
}

impl FederationEnvelope {
    pub fn new_request(
        from_dept_id: impl Into<String>,
        to_dept_id: impl Into<String>,
        consent_grant_ids: Vec<Uuid>,
        payload_encrypted: String,
        signature: String,
    ) -> Self {
        Self {
            message_id: Uuid::new_v4(),
            correlation_id: None,
            message_type: FederationMessageType::DataRequest,
            from_dept_id: from_dept_id.into(),
            to_dept_id: to_dept_id.into(),
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            consent_grant_ids,
            payload_encrypted,
            signature,
        }
    }

    fn payload_hash(&self) -> [u8; 32] {
        let payload = EnvelopeSignPayload {
            message_id: &self.message_id,
            correlation_id: self.correlation_id,
            message_type: &self.message_type,
            from_dept_id: &self.from_dept_id,
            to_dept_id: &self.to_dept_id,
            timestamp_ms: self.timestamp_ms,
            consent_grant_ids: &self.consent_grant_ids,
            payload_encrypted: &self.payload_encrypted,
        };
        let json = serde_json::to_vec(&payload).expect("serialisation must not fail");
        Sha256::digest(&json).into()
    }

    /// Sign this envelope with the sender's Ed25519 keypair.
    /// Sets the `signature` field to the base64-encoded signature.
    pub fn sign(&mut self, signing_key: &SigningKey) {
        use base64::Engine;
        let hash = self.payload_hash();
        let sig = signing_key.sign(&hash);
        self.signature = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());
    }

    /// Verify the envelope's signature against the sender's public key.
    pub fn verify(&self, sender_key: &VerifyingKey) -> Result<(), FederationError> {
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
        sender_key
            .verify(&hash, &sig)
            .map_err(|_| FederationError::SignatureInvalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_envelope() -> FederationEnvelope {
        FederationEnvelope {
            message_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            correlation_id: None,
            message_type: FederationMessageType::DataRequest,
            from_dept_id: "winz".into(),
            to_dept_id: "ird".into(),
            timestamp_ms: 1_700_000_000_000,
            consent_grant_ids: vec![Uuid::parse_str("00000000-0000-0000-0000-000000000099").unwrap()],
            payload_encrypted: "encrypted-payload".into(),
            signature: String::new(),
        }
    }

    #[test]
    fn sign_and_verify_round_trip() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let mut env = make_test_envelope();
        env.sign(&signing_key);

        assert!(!env.signature.is_empty());
        assert!(env.verify(&verifying_key).is_ok());
    }

    #[test]
    fn verify_fails_with_wrong_key() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let wrong_key = SigningKey::generate(&mut OsRng);

        let mut env = make_test_envelope();
        env.sign(&signing_key);

        assert!(env.verify(&wrong_key.verifying_key()).is_err());
    }

    #[test]
    fn verify_fails_with_tampered_payload() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let mut env = make_test_envelope();
        env.sign(&signing_key);
        assert!(env.verify(&verifying_key).is_ok());

        // Tamper with the payload
        env.to_dept_id = "moh".into();
        assert!(env.verify(&verifying_key).is_err());
    }
}
