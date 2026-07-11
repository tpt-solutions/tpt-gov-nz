pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawNzdfBatch};

/// Pluggable ingestion source for the New Zealand Defence Force ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawNzdfBatch, IngestError>;
}
