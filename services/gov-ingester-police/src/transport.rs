pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawPoliceBatch};

/// Pluggable ingestion source for the Police ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawPoliceBatch, IngestError>;
}
