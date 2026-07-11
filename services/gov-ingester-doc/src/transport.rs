pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawDocBatch};

/// Pluggable ingestion source for the DOC ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawDocBatch, IngestError>;
}
