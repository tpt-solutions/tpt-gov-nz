use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Invalid DID format: {0}")]
    InvalidDid(String),

    #[error("Credential signature invalid")]
    InvalidSignature,

    #[error("Credential expired")]
    Expired,

    #[error("Credential revoked")]
    Revoked,

    #[error("Serialisation error: {0}")]
    Serialisation(#[from] serde_json::Error),
}
