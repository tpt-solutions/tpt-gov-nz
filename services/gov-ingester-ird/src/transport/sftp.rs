use std::path::PathBuf;

use async_trait::async_trait;

use crate::{
    error::IngestError,
    raw::RawIrdBatch,
    transport::IngesterTransport,
};

/// Connection details for the real IRD legacy batch-file integration.
///
/// In production the ingester connects to IRD's SFTP endpoint, downloads the
/// periodic batch extract, and parses it. This struct carries the configuration;
/// the [`IngesterTransport::pull`] implementation is intentionally a stub for
/// Phase 1 and returns [`IngestError::NotImplemented`].
#[derive(Debug, Clone)]
pub struct SftpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub private_key_path: PathBuf,
    pub remote_path: String,
}

pub struct SftpTransport {
    config: SftpConfig,
}

impl SftpTransport {
    pub fn new(config: SftpConfig) -> Self {
        Self { config }
    }

    /// Future production implementation outline:
    /// 1. Establish an SSH session with `config.private_key_path`.
    /// 2. Open an SFTP channel and stream `config.remote_path` to a temp file.
    /// 3. Parse the downloaded batch exactly like [`super::mock::MockTransport`].
    #[allow(dead_code)]
    fn connection_summary(&self) -> String {
        format!(
            "sftp://{}@{}:{}{}",
            self.config.username, self.config.host, self.config.port, self.config.remote_path
        )
    }
}

#[async_trait]
impl IngesterTransport for SftpTransport {
    async fn pull(&self) -> Result<RawIrdBatch, IngestError> {
        Err(IngestError::NotImplemented(format!(
            "SFTP transport not yet wired up (target: {}). \
             Implement SSH/SFTP download in production; the transform + upsert \
             pipeline is transport-agnostic and ready to consume the result.",
            self.connection_summary()
        )))
    }
}
