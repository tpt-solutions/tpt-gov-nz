pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawCorrectionsBatch};

/// Pluggable ingestion source for the Corrections ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawCorrectionsBatch, IngestError>;
}
