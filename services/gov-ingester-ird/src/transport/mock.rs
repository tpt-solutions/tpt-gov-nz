use std::path::PathBuf;

use async_trait::async_trait;

use crate::{
    error::IngestError,
    raw::RawIrdBatch,
    transport::IngesterTransport,
};

/// Reads a raw IRD batch from a JSON fixture file on disk.
///
/// Used in dev/demo and in CI so the full ingestion pipeline can run with no
/// legacy IRD system attached. Point `IRD_MOCK_FIXTURE` at any `RawIrdBatch` JSON.
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
    async fn pull(&self) -> Result<RawIrdBatch, IngestError> {
        let bytes = tokio::fs::read(&self.fixture_path).await.map_err(|e| {
            IngestError::Transport(format!(
                "failed to read fixture {}: {e}",
                self.fixture_path.display()
            ))
        })?;
        let batch = serde_json::from_slice::<RawIrdBatch>(&bytes)
            .map_err(|e| IngestError::Transform(format!("invalid fixture JSON: {e}")))?;
        Ok(batch)
    }
}
