pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawMpiBatch};

/// Pluggable ingestion source for the MPI ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawMpiBatch, IngestError>;
}
