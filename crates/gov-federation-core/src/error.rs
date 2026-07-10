use thiserror::Error;

#[derive(Error, Debug)]
pub enum FederationError {
    #[error("Invalid consent grant: {0}")]
    InvalidConsent(String),

    #[error("Signature verification failed")]
    SignatureInvalid,

    #[error("Scope not granted: {0}")]
    ScopeNotGranted(String),

    #[error("Grant expired")]
    GrantExpired,

    #[error("Unknown department: {0}")]
    UnknownDepartment(String),

    #[error("Serialisation error: {0}")]
    Serialisation(#[from] serde_json::Error),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Transport error: {0}")]
    Transport(String),
}
