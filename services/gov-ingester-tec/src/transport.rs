pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawTecBatch};

/// Pluggable ingestion source for the Tertiary Education Commission ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawTecBatch, IngestError>;
}
