pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawWorksafeBatch};

/// Pluggable ingestion source for the WorkSafe New Zealand ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawWorksafeBatch, IngestError>;
}
