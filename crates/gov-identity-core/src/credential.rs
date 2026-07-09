use crate::{did::GovDid, error::IdentityError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
}
