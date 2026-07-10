pub mod mock;
pub mod sftp;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawIrdBatch};

/// Pluggable ingestion source. The ingester logic (`run_once`) is written against
/// this trait only, so the transport (mock file vs. real SFTP) can be swapped
/// without touching the transform/upsert pipeline.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    /// Pull the next batch of raw IRD data.
    async fn pull(&self) -> Result<RawIrdBatch, IngestError>;
}
