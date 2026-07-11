pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawMoeBatch};

/// Pluggable ingestion source for the Ministry of Education ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawMoeBatch, IngestError>;
}
