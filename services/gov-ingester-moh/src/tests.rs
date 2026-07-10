//! Integration + unit tests for the MOH ingester.
//!
//! Run with a Postgres available and pointing `DATABASE_URL` at the MOH dept DB:
//!
//! ```sh
//! DATABASE_URL=postgresql://moh:moh_dev_password@localhost:5432/moh \
//!   cargo test -p gov-ingester-moh
//! ```

use std::path::PathBuf;

use crate::{
    db, ingest, models::IngestionStatus, raw::RawMohBatch, transform::transform_citizen,
    transport::mock::MockTransport,
};

#[sqlx::test(migrations = "../gov-dept-moh/migrations")]
async fn upsert_citizen_is_idempotent(pool: sqlx::PgPool) {
    let (_, first) = db::upsert_citizen(&pool, "did:gov:nz:t", "ABC1234").await.unwrap();
    assert!(first);
    let (_, second) = db::upsert_citizen(&pool, "did:gov:nz:t", "ABC1234").await.unwrap();
    assert!(!second);
}

#[sqlx::test(migrations = "../gov-dept-moh/migrations")]
async fn upsert_prescription_is_idempotent(pool: sqlx::PgPool) {
    let (cid, _) = db::upsert_citizen(&pool, "did:gov:nz:p", "DEF5678").await.unwrap();
    let e = crate::models::PrescriptionEntity {
        medication: "Atorvastatin".into(),
        dose: "20mg".into(),
        repeats_total: 3,
        repeats_remaining: 2,
        issued_at: chrono::NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
    };
    assert!(db::upsert_prescription(&pool, cid, &e).await.unwrap());
    assert!(!db::upsert_prescription(&pool, cid, &e).await.unwrap());
}

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/moh_batch.json")
}

#[sqlx::test(migrations = "../gov-dept-moh/migrations")]
async fn run_once_with_mock_transport(pool: sqlx::PgPool) {
    let transport = MockTransport::new(fixture_path());
    let summary = ingest::run_once(&pool, &transport).await.unwrap();

    assert_eq!(summary.citizens_processed, 2);
    assert_eq!(summary.status, IngestionStatus::Success);
    // test-citizen-001 is seeded (rows are updates); test-citizen-002 is new (inserts).
    assert_eq!(summary.rows_inserted, 4);
    assert_eq!(summary.rows_updated, 5);
    assert!(summary.batch_id.as_deref() == Some("MOH-2026-07-B001"));

    let run = db::latest_run(&pool).await.unwrap().expect("audit row");
    assert_eq!(run.0, "mock");
    assert_eq!(run.1, 2);
    assert_eq!(run.3, "success");
}

#[sqlx::test(migrations = "../gov-dept-moh/migrations")]
async fn run_once_is_repeatable(pool: sqlx::PgPool) {
    let transport = MockTransport::new(fixture_path());
    let first = ingest::run_once(&pool, &transport).await.unwrap();
    let second = ingest::run_once(&pool, &transport).await.unwrap();
    assert_eq!(second.rows_inserted, 0);
    assert!(second.rows_updated > first.rows_updated || second.rows_updated >= 1);
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMohBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);
    let t = transform_citizen(&batch.citizens[1]).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t.citizen.nhi, "ZZA9876");
    assert_eq!(t.vaccinations.len(), 1);
}
