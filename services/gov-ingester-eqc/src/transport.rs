pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawEqcBatch};

/// Pluggable ingestion source for the Earthquake Commission (Toka Tū Ake) ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawEqcBatch, IngestError>;
}
