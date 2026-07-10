//! Integration + unit tests for the WINZ ingester.
//!
//! Run with a Postgres available and pointing `DATABASE_URL` at the WINZ dept DB
//! (migrations are applied by `sqlx::test` from the shared dept migrations):
//!
//! ```sh
//! DATABASE_URL=postgresql://winz:winz_dev_password@localhost:5432/winz \
//!   cargo test -p gov-ingester-winz
//! ```

use std::path::PathBuf;

use crate::{
    db,
    ingest,
    models::IngestionStatus,
    raw::RawWinzBatch,
    transform::transform_citizen,
    transport::mock::MockTransport,
};

#[sqlx::test(migrations = "../gov-dept-winz/migrations")]
async fn upsert_citizen_is_idempotent(pool: sqlx::PgPool) {
    let (_, first) = db::upsert_citizen(&pool, "did:gov:nz:t", "WINZ-CLIENT-X")
        .await
        .unwrap();
    assert!(first, "first upsert should insert");

    let (_, second) = db::upsert_citizen(&pool, "did:gov:nz:t", "WINZ-CLIENT-X")
        .await
        .unwrap();
    assert!(!second, "second upsert should update, not insert");
}

#[sqlx::test(migrations = "../gov-dept-winz/migrations")]
async fn upsert_benefit_is_idempotent(pool: sqlx::PgPool) {
    let (cid, _) = db::upsert_citizen(&pool, "did:gov:nz:b", "WINZ-CLIENT-B")
        .await
        .unwrap();

    let e = crate::models::BenefitEntity {
        benefit_type: "jobseeker".into(),
        weekly_amount: sqlx::types::Decimal::from(275),
        start_date: None,
        review_date: None,
        status: "active".into(),
    };
    assert!(db::upsert_benefit(&pool, cid, &e).await.unwrap());
    assert!(!db::upsert_benefit(&pool, cid, &e).await.unwrap());
}

// ── full run with mock transport ────────────────────────────────────────────────

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/winz_batch.json")
}

#[sqlx::test(migrations = "../gov-dept-winz/migrations")]
async fn run_once_with_mock_transport(pool: sqlx::PgPool) {
    let transport = MockTransport::new(fixture_path());
    let summary = ingest::run_once(&pool, &transport).await.unwrap();

    assert_eq!(summary.citizens_processed, 2);
    assert_eq!(summary.status, IngestionStatus::Success);
    // test-citizen-001 is seeded by migration 007 (so its rows are updates);
    // test-citizen-002 is new (so its rows are inserts).
    assert_eq!(summary.rows_inserted, 6);
    assert_eq!(summary.rows_updated, 6);
    assert!(summary.batch_id.as_deref() == Some("WINZ-2026-07-B001"));

    // Audit row was written.
    let run = db::latest_run(&pool).await.unwrap().expect("audit row");
    assert_eq!(run.0, "mock");
    assert_eq!(run.1, 2);
    assert_eq!(run.3, "success");

    // Data landed in the dept tables.
    let citizen = db::upsert_citizen(&pool, "did:gov:nz:test-citizen-002", "WINZ-CLIENT-002")
        .await
        .unwrap();
    assert!(!citizen.1, "already upserted by the run");

    let amount = sqlx::query_scalar!(
        "SELECT weekly_amount FROM benefits WHERE citizen_id = $1 AND benefit_type = 'sole-parent'",
        citizen.0
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(amount, sqlx::types::Decimal::try_from(389.10).unwrap());
}

#[sqlx::test(migrations = "../gov-dept-winz/migrations")]
async fn run_once_is_repeatable(pool: sqlx::PgPool) {
    let transport = MockTransport::new(fixture_path());
    let first = ingest::run_once(&pool, &transport).await.unwrap();
    let second = ingest::run_once(&pool, &transport).await.unwrap();

    // Second run is purely updates (idempotent) — no new citizen inserted.
    assert_eq!(second.rows_inserted, 0);
    assert!(second.rows_updated > first.rows_updated || second.rows_updated >= 1);
}

// ── transform ↔ raw round trip via real fixture file ────────────────────────────

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawWinzBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);
    let t = transform_citizen(&batch.citizens[1]).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t.benefits.len(), 2);
    assert_eq!(t.payments.len(), 2);
}
