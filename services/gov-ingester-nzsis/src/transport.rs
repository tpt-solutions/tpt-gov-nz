pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawNzsisBatch};

/// Pluggable ingestion source for the New Zealand Security Intelligence Service ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawNzsisBatch, IngestError>;
}
