pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawEroBatch};

/// Pluggable ingestion source for the Education Review Office ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawEroBatch, IngestError>;
}
