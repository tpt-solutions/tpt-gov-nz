use std::time::Duration;

use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use gov_ingester_mch::{
    config::{Config, TransportKind},
    ingest,
    transport::{legacy::LegacyTransport, mock::MockTransport, IngesterTransport},
    IngestError,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("gov_ingester_mch=info,sqlx=warn")),
        )
        .json()
        .init();

    let config = Config::from_env().map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let pool = PgPool::connect(&config.database_url).await?;

    sqlx::migrate!("../gov-dept-mch/migrations")
        .run(&pool)
        .await?;
    info!("Migrations applied");

    let transport: Box<dyn IngesterTransport> = match config.transport {
        TransportKind::Mock => Box::new(MockTransport::new(&config.mock_fixture)),
        TransportKind::Legacy => Box::new(LegacyTransport::new(config.legacy.clone())),
    };

    if config.run_once {
        let summary = ingest::run_once_audited(&pool, transport.as_ref())
            .await
            .map_err(|e: IngestError| anyhow::anyhow!(e.to_string()))?;
        info!(
            source = %summary.source,
            citizens = summary.citizens_processed,
            inserted = summary.rows_inserted,
            updated = summary.rows_updated,
            "ingestion pass complete"
        );
        return Ok(());
    }

    let mut interval = tokio::time::interval(Duration::from_secs(config.interval_secs));
    info!(
        interval_secs = config.interval_secs,
        transport = ?config.transport,
        "ingester scheduler started"
    );
    loop {
        interval.tick().await;
        match ingest::run_once_audited(&pool, transport.as_ref()).await {
            Ok(summary) => info!(
                source = %summary.source,
                citizens = summary.citizens_processed,
                inserted = summary.rows_inserted,
                updated = summary.rows_updated,
                "ingestion pass complete"
            ),
            Err(e) => error!(error = %e, "ingestion pass failed"),
        }
    }
}
