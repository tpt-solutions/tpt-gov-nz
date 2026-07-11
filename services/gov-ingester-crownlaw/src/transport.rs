pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawCrownlawBatch};

/// Pluggable ingestion source for the Crown Law Office ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawCrownlawBatch, IngestError>;
}
