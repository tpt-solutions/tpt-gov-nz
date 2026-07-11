pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawStatsnzBatch};

/// Pluggable ingestion source for the Statistics New Zealand ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawStatsnzBatch, IngestError>;
}
