pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawFenzBatch};

/// Pluggable ingestion source for the Fire and Emergency New Zealand ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawFenzBatch, IngestError>;
}
