//! Ingestion orchestration — transport-agnostic.
//!
//! `run_once` pulls a batch, transforms each citizen, and idempotently upserts
//! the result into the department DB. Every pass is recorded in the ingestion
//! audit log regardless of outcome.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db,
    error::IngestError,
    models::{new_run_id, IngestionStatus, IngestionSummary},
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
            db::upsert_citizen(pool, &t.citizen.did, &t.citizen.client_id).await?;
        if was_inserted {
            inserted += 1;
        } else {
            updated += 1;
        }

        for b in &t.benefits {
            count(db::upsert_benefit(pool, citizen_id, b).await?, &mut inserted, &mut updated);
        }
        for p in &t.payments {
            count(db::upsert_payment(pool, citizen_id, p).await?, &mut inserted, &mut updated);
        }
    }

    let citizens_processed = batch.citizens.len() as u32;
    let finished = Utc::now();

    db::record_ingestion_run(
        pool,
        run_id,
        &source,
        batch_id.as_deref(),
        started,
        finished,
        citizens_processed as i32,
        inserted as i32,
        updated as i32,
        IngestionStatus::Success,
        None,
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

/// Run a single pass and always record a `failed` audit row if it errors.
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
            // Best-effort: record the failure. If even this fails, propagate original.
            db::record_ingestion_run(
                pool,
                run_id,
                "unknown",
                None,
                now,
                now,
                0,
                0,
                0,
                IngestionStatus::Failed,
                Some(&msg),
            )
            .await
            .ok();
            Err(e)
        }
    }
}
