use std::path::PathBuf;

use async_trait::async_trait;

use crate::{
    error::IngestError,
    raw::RawMfatBatch,
    transport::IngesterTransport,
};

/// Reads a raw Ministry of Foreign Affairs and Trade batch from a JSON fixture file on disk (dev/demo/CI).
pub struct MockTransport {
    fixture_path: PathBuf,
}

impl MockTransport {
    pub fn new(fixture_path: impl Into<PathBuf>) -> Self {
        Self {
            fixture_path: fixture_path.into(),
        }
    }
}

#[async_trait]
impl IngesterTransport for MockTransport {
    async fn pull(&self) -> Result<RawMfatBatch, IngestError> {
        let bytes = tokio::fs::read(&self.fixture_path).await.map_err(|e| {
            IngestError::Transport(format!(
                "failed to read fixture {}: {e}",
                self.fixture_path.display()
            ))
        })?;
        let batch = serde_json::from_slice::<RawMfatBatch>(&bytes)
            .map_err(|e| IngestError::Transform(format!("invalid fixture JSON: {e}")))?;
        Ok(batch)
    }
}
