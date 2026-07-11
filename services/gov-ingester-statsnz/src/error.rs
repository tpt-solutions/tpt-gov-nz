#[derive(Debug, thiserror::Error)]
pub enum IngestError {
    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Transform error: {0}")]
    Transform(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Transport not implemented: {0}")]
    NotImplemented(String),

    #[error("Config error: {0}")]
    Config(String),
}
