pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawPublicserviceBatch};

/// Pluggable ingestion source for the Te Kawa Mataaho Public Service Commission ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawPublicserviceBatch, IngestError>;
}
