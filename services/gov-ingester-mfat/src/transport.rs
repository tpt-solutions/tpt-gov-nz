pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawMfatBatch};

/// Pluggable ingestion source for the Ministry of Foreign Affairs and Trade ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawMfatBatch, IngestError>;
}
