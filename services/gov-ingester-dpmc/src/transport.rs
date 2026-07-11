pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawDpmcBatch};

/// Pluggable ingestion source for the Department of the Prime Minister and Cabinet ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawDpmcBatch, IngestError>;
}
