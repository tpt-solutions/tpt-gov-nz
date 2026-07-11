pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawWomenBatch};

/// Pluggable ingestion source for the Ministry for Women ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawWomenBatch, IngestError>;
}
