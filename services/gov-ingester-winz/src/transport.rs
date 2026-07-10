pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawWinzBatch};

/// Pluggable ingestion source. The ingester logic (`run_once`) is written against
/// this trait only, so the transport (mock file vs. real WINZ legacy system) can
/// be swapped without touching the transform/upsert pipeline.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    /// Pull the next batch of raw WINZ data.
    async fn pull(&self) -> Result<RawWinzBatch, IngestError>;
}
