pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::RawRetirementBatch};

/// Pluggable ingestion source for the Retirement Commission (Te Ara Ahunga Ora) ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<RawRetirementBatch, IngestError>;
}
