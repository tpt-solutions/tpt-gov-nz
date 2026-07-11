pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawMaritimeBatch};

/// Pluggable ingestion source for the Maritime New Zealand ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawMaritimeBatch, IngestError>;
}
