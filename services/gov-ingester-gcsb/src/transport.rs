pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawGcsbBatch};

/// Pluggable ingestion source for the Government Communications Security Bureau ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawGcsbBatch, IngestError>;
}
