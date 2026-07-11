pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawNzqaBatch};

/// Pluggable ingestion source for the NZQA ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawNzqaBatch, IngestError>;
}
