pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawTearawhitiBatch};

/// Pluggable ingestion source for the Te Arawhiti ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawTearawhitiBatch, IngestError>;
}
