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
            db::upsert_citizen(pool, &t.citizen.did, &t.citizen.ird_number).await?;
        if was_inserted {
            inserted += 1;
        } else {
            updated += 1;
        }

        if let Some(e) = &t.income {
            count(
                db::upsert_income(pool, citizen_id, e).await?,
                &mut inserted,
                &mut updated,
            );
        }
        if let Some(e) = &t.tax {
            count(
                db::upsert_tax_assessment(pool, citizen_id, e).await?,
                &mut inserted,
                &mut updated,
            );
        }
        count(
            db::upsert_gst_registration(pool, citizen_id, &t.gst).await?,
            &mut inserted,
            &mut updated,
        );
        if let Some(e) = &t.kiwisaver {
            count(
                db::upsert_kiwisaver(pool, citizen_id, e).await?,
                &mut inserted,
                &mut updated,
            );
        }
        if let Some(e) = &t.wff {
            count(
                db::upsert_wff(pool, citizen_id, e).await?,
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
