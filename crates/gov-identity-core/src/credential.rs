use crate::{did::GovDid, error::IdentityError};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Canonical JSON fields used for signing a VerifiableCredential.
/// The `proof` field itself is excluded from the signed payload.
#[derive(Serialize)]
struct VcSignPayload<'a> {
    id: &'a Uuid,
    issuer_did: &'a str,
    subject_did: &'a str,
    credential_type: &'a str,
    issued_at: i64,
    expires_at: i64,
    claims: &'a serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub id: Uuid,
    pub issuer_did: GovDid,
    pub subject_did: GovDid,
    pub credential_type: String,
    pub issued_at: i64,
    pub expires_at: i64,
    pub claims: serde_json::Value,
    pub proof: String,
}

impl VerifiableCredential {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.expires_at
    }

    pub fn verify_expiry(&self) -> Result<(), IdentityError> {
        if self.is_expired() {
            Err(IdentityError::Expired)
        } else {
            Ok(())
        }
    }

    /// Compute the SHA-256 hash of the canonical sign payload.
    fn payload_hash(&self) -> [u8; 32] {
        let payload = VcSignPayload {
            id: &self.id,
            issuer_did: self.issuer_did.as_str(),
            subject_did: self.subject_did.as_str(),
            credential_type: &self.credential_type,
            issued_at: self.issued_at,
            expires_at: self.expires_at,
            claims: &self.claims,
        };
        let json = serde_json::to_vec(&payload).expect("serialisation of sign payload must not fail");
        Sha256::digest(&json).into()
    }

    /// Verify the Ed25519 signature against the issuer's public key.
    pub fn verify_signature(&self, issuer_key: &VerifyingKey) -> Result<(), IdentityError> {
        let hash = self.payload_hash();
        let sig_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &self.proof,
        )
        .map_err(|e| IdentityError::Crypto(format!("invalid base64 proof: {e}")))?;
        let sig = ed25519_dalek::Signature::from_bytes(
            &sig_bytes.try_into().map_err(|_| IdentityError::Crypto("proof length invalid".into()))?,
        );
        issuer_key
            .verify(&hash, &sig)
            .map_err(|_| IdentityError::InvalidSignature)
    }
}

/// Canonical JSON fields used for signing a DataGrantCredential.
#[derive(Serialize)]
struct DgcSignPayload<'a> {
    id: &'a Uuid,
    citizen_did: &'a str,
    requesting_dept_id: &'a str,
    providing_dept_id: &'a str,
    scopes: &'a [String],
    issued_at: i64,
    expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataGrantCredential {
    pub id: Uuid,
    pub citizen_did: GovDid,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub scopes: Vec<String>,
    pub issued_at: i64,
    pub expires_at: i64,
    pub signature: String,
}

impl DataGrantCredential {
    pub fn is_valid(&self) -> Result<(), IdentityError> {
        if chrono::Utc::now().timestamp() > self.expires_at {
            return Err(IdentityError::Expired);
        }
        Ok(())
    }

    fn payload_hash(&self) -> [u8; 32] {
        let payload = DgcSignPayload {
            id: &self.id,
            citizen_did: self.citizen_did.as_str(),
            requesting_dept_id: &self.requesting_dept_id,
            providing_dept_id: &self.providing_dept_id,
            scopes: &self.scopes,
            issued_at: self.issued_at,
            expires_at: self.expires_at,
        };
        let json = serde_json::to_vec(&payload).expect("serialisation of sign payload must not fail");
        Sha256::digest(&json).into()
    }

    /// Verify the Ed25519 signature against the issuer's public key.
    pub fn verify_signature(&self, issuer_key: &VerifyingKey) -> Result<(), IdentityError> {
        let hash = self.payload_hash();
        let sig_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &self.signature,
        )
        .map_err(|e| IdentityError::Crypto(format!("invalid base64 signature: {e}")))?;
        let sig = ed25519_dalek::Signature::from_bytes(
            &sig_bytes.try_into().map_err(|_| IdentityError::Crypto("signature length invalid".into()))?,
        );
        issuer_key
            .verify(&hash, &sig)
            .map_err(|_| IdentityError::InvalidSignature)
    }
}

/// The identity server's keypair — used to issue and sign credentials.
pub struct CredentialIssuer {
    signing_key: SigningKey,
    pub did: GovDid,
}

impl CredentialIssuer {
    pub fn generate() -> Self {
        use rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut OsRng);
        let did = GovDid::from_verifying_key(&signing_key.verifying_key());
        Self { signing_key, did }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    /// Sign a payload hash with the issuer's key and return base64.
    fn sign_hash(&self, hash: &[u8; 32]) -> String {
        use base64::Engine;
        let sig = self.signing_key.sign(hash);
        base64::engine::general_purpose::STANDARD.encode(sig.to_bytes())
    }

    /// Issue a VerifiableCredential, signing the proof field.
    pub fn issue_vc(
        &self,
        subject_did: GovDid,
        credential_type: impl Into<String>,
        claims: serde_json::Value,
        ttl_secs: i64,
    ) -> VerifiableCredential {
        let now = chrono::Utc::now().timestamp();
        let mut vc = VerifiableCredential {
            id: Uuid::new_v4(),
            issuer_did: self.did.clone(),
            subject_did,
            credential_type: credential_type.into(),
            issued_at: now,
            expires_at: now + ttl_secs,
            claims,
            proof: String::new(),
        };
        vc.proof = self.sign_hash(&vc.payload_hash());
        vc
    }

    /// Issue a DataGrantCredential, signing the signature field.
    pub fn issue_data_grant(
        &self,
        citizen_did: GovDid,
        requesting_dept_id: impl Into<String>,
        providing_dept_id: impl Into<String>,
        scopes: Vec<String>,
        ttl_secs: i64,
    ) -> DataGrantCredential {
        let now = chrono::Utc::now().timestamp();
        let mut dgc = DataGrantCredential {
            id: Uuid::new_v4(),
            citizen_did,
            requesting_dept_id: requesting_dept_id.into(),
            providing_dept_id: providing_dept_id.into(),
            scopes,
            issued_at: now,
            expires_at: now + ttl_secs,
            signature: String::new(),
        };
        dgc.signature = self.sign_hash(&dgc.payload_hash());
        dgc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vc_round_trip_sign_and_verify() {
        let issuer = CredentialIssuer::generate();
        let subject = GovDid::parse("did:gov:nz:subject123").unwrap();

        let vc = issuer.issue_vc(
            subject,
            "IdentityVerification",
            serde_json::json!({"name": "Alex Tane"}),
            3600,
        );

        assert!(!vc.is_expired());
        assert!(vc.verify_signature(&issuer.verifying_key()).is_ok());
    }

    #[test]
    fn vc_verify_fails_with_wrong_key() {
        let issuer = CredentialIssuer::generate();
        let wrong_issuer = CredentialIssuer::generate();
        let subject = GovDid::parse("did:gov:nz:subject456").unwrap();

        let vc = issuer.issue_vc(
            subject,
            "IdentityVerification",
            serde_json::json!({}),
            3600,
        );

        assert!(vc.verify_signature(&wrong_issuer.verifying_key()).is_err());
    }

    #[test]
    fn dgc_round_trip_sign_and_verify() {
        let issuer = CredentialIssuer::generate();
        let citizen = GovDid::parse("did:gov:nz:citizen001").unwrap();

        let dgc = issuer.issue_data_grant(
            citizen,
            "winz",
            "ird",
            vec!["ird:income".into(), "ird:tax-summary".into()],
            86400,
        );

        assert!(dgc.is_valid().is_ok());
        assert!(dgc.verify_signature(&issuer.verifying_key()).is_ok());
    }

    #[test]
    fn dgc_verify_fails_with_wrong_key() {
        let issuer = CredentialIssuer::generate();
        let wrong_issuer = CredentialIssuer::generate();
        let citizen = GovDid::parse("did:gov:nz:citizen002").unwrap();

        let dgc = issuer.issue_data_grant(
            citizen,
            "moh",
            "ird",
            vec!["ird:income".into()],
            86400,
        );

        assert!(dgc.verify_signature(&wrong_issuer.verifying_key()).is_err());
    }
}
