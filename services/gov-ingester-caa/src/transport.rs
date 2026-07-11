pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawCaaBatch};

/// Pluggable ingestion source for the Civil Aviation Authority ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawCaaBatch, IngestError>;
}
