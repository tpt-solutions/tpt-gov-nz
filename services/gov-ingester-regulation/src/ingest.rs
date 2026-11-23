//! Ingestion orchestration — transport-agnostic.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db,
    error::IngestError,
    models::{IngestionStatus, IngestionSummary, new_run_id},
    transform::transform_citizen,
    transport::IngesterTransport,
};

pub async fn run_once(
    pool: &PgPool,
    transport: &dyn IngesterTransport,
) -> Result<IngestionSummary, IngestError> {
    let run_id: Uuid = new_run_id();
    let started = Utc::now();

    let batch = transport.pull().await?;
    let source = batch.source.clone();
    let batch_id = Some(batch.batch_id.clone());

    let mut inserted = 0u32;
    let mut updated = 0u32;

    for raw in &batch.citizens {
        let t = transform_citizen(raw)?;

        let (citizen_id, was_inserted) =
            db::upsert_citizen(pool, &t.citizen.did, &t.citizen.regulation_id).await?;
        if was_inserted {
            inserted += 1;
        } else {
            updated += 1;
        }

        for regulatory_reviews in &t.regulatory_reviews {
            count(
                db::upsert_regulatory_reviews(pool, citizen_id, regulatory_reviews).await?,
                &mut inserted,
                &mut updated,
            );
        }

        for proposals in &t.proposals {
            count(
                db::upsert_proposals(pool, citizen_id, proposals).await?,
                &mut inserted,
                &mut updated,
            );
        }
    }

    let citizens_processed = batch.citizens.len() as u32;
    let finished = Utc::now();

    db::record_ingestion_run(
        pool,
        &db::IngestionRunRecord {
        run_id,
        source: &source,
        batch_id: batch_id.as_deref(),
        started_at: started,
        finished_at: finished,
        citizens_processed: citizens_processed as i32,
        rows_inserted: inserted as i32,
        rows_updated: updated as i32,
        status: IngestionStatus::Success,
        error_message: None,
        },
        )
    .await?;

    Ok(IngestionSummary {
        source,
        batch_id,
        citizens_processed,
        rows_inserted: inserted,
        rows_updated: updated,
        status: IngestionStatus::Success,
        error_message: None,
    })
}

#[inline]
fn count(was_inserted: bool, inserted: &mut u32, updated: &mut u32) {
    if was_inserted {
        *inserted += 1;
    } else {
        *updated += 1;
    }
}

pub async fn run_once_audited(
    pool: &PgPool,
    transport: &dyn IngesterTransport,
) -> Result<IngestionSummary, IngestError> {
    match run_once(pool, transport).await {
        Ok(summary) => Ok(summary),
        Err(e) => {
            let run_id = new_run_id();
            let now = Utc::now();
            let msg = e.to_string();
            db::record_ingestion_run(
        pool,
        &db::IngestionRunRecord {
        run_id,
        source: "unknown",
        batch_id: None,
        started_at: now,
        finished_at: now,
        citizens_processed: 0,
        rows_inserted: 0,
        rows_updated: 0,
        status: IngestionStatus::Failed,
        error_message: Some(&msg),
        },
        )
            .await
            .ok();
            Err(e)
        }
    }
}
