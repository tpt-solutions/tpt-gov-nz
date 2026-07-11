use std::path::PathBuf;

use async_trait::async_trait;

use crate::{
    error::IngestError,
    raw::RawTreasuryBatch,
    transport::IngesterTransport,
};

/// Connection details for the real The Treasury legacy system integration.
#[derive(Debug, Clone)]
pub struct LegacyConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub private_key_path: PathBuf,
    pub remote_path: String,
}

pub struct LegacyTransport {
    config: LegacyConfig,
}

impl LegacyTransport {
    pub fn new(config: LegacyConfig) -> Self {
        Self { config }
    }

    #[allow(dead_code)]
    fn connection_summary(&self) -> String {
        format!(
            "treasury-legacy://{}@{}:{}{}",
            self.config.username, self.config.host, self.config.port, self.config.remote_path
        )
    }
}

#[async_trait]
impl IngesterTransport for LegacyTransport {
    async fn pull(&self) -> Result<RawTreasuryBatch, IngestError> {
        Err(IngestError::NotImplemented(format!(
            "The Treasury legacy transport not yet wired up (target: {}).              Implement the legacy system download in production; the transform +              upsert pipeline is transport-agnostic and ready to consume the result.",
            self.connection_summary()
        )))
    }
}
