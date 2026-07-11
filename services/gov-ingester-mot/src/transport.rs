pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawMotBatch};

/// Pluggable ingestion source for the Ministry of Transport ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawMotBatch, IngestError>;
}
