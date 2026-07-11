//! Integration + unit tests for the NZTA ingester.
//!
//! Run with a Postgres available and pointing `DATABASE_URL` at the NZTA dept DB:
//!
//! ```sh
//! DATABASE_URL=postgresql://nzta:nzta_dev_password@localhost:5432/nzta \
//!   cargo test -p gov-ingester-nzta
//! ```

use std::path::PathBuf;

use crate::{
    db, ingest, models::IngestionStatus, raw::RawNztaBatch, transform::transform_citizen,
    transport::mock::MockTransport,
};

#[sqlx::test(migrations = "../gov-dept-nzta/migrations")]
async fn upsert_citizen_is_idempotent(pool: sqlx::PgPool) {
    let (_, first) = db::upsert_citizen(&pool, "did:gov:nz:t", "NZ9999").await.unwrap();
    assert!(first);
    let (_, second) = db::upsert_citizen(&pool, "did:gov:nz:t", "NZ9999").await.unwrap();
    assert!(!second);
}

#[sqlx::test(migrations = "../gov-dept-nzta/migrations")]
async fn upsert_driver_licence_is_idempotent(pool: sqlx::PgPool) {
    let (cid, _) = db::upsert_citizen(&pool, "did:gov:nz:p", "NZ8888").await.unwrap();
    let e = crate::models::DriverLicenceEntity {
        licence_number: "NZ8888".into(),
        full_name: "Test Driver".into(),
        licence_class: "1 (car)".into(),
        expiry_date: chrono::NaiveDate::from_ymd_opt(2028, 1, 1).unwrap(),
        conditions: None,
    };
    assert!(db::upsert_driver_licence(&pool, cid, &e).await.unwrap());
    assert!(!db::upsert_driver_licence(&pool, cid, &e).await.unwrap());
}

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/nzta_batch.json")
}

#[sqlx::test(migrations = "../gov-dept-nzta/migrations")]
async fn run_once_with_mock_transport(pool: sqlx::PgPool) {
    let transport = MockTransport::new(fixture_path());
    let summary = ingest::run_once(&pool, &transport).await.unwrap();

    assert_eq!(summary.citizens_processed, 2);
    assert_eq!(summary.status, IngestionStatus::Success);
    // Both citizens + their records are pre-seeded by migration 007, so this is a pure
    // idempotent update pass.
    assert_eq!(summary.rows_inserted, 0);
    assert!(summary.rows_updated > 0);
    assert!(summary.batch_id.as_deref() == Some("NZTA-2026-07-B001"));

    let run = db::latest_run(&pool).await.unwrap().expect("audit row");
    assert_eq!(run.0, "mock");
    assert_eq!(run.1, 2);
    assert_eq!(run.3, "success");
}

#[sqlx::test(migrations = "../gov-dept-nzta/migrations")]
async fn run_once_is_repeatable(pool: sqlx::PgPool) {
    let transport = MockTransport::new(fixture_path());
    let first = ingest::run_once(&pool, &transport).await.unwrap();
    let second = ingest::run_once(&pool, &transport).await.unwrap();
    assert_eq!(second.rows_inserted, 0);
    assert!(second.rows_updated >= first.rows_updated || second.rows_updated >= 1);
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawNztaBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);
    let t = transform_citizen(&batch.citizens[1]).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t.citizen.driver_licence_number, "NZ7654321");
    assert_eq!(t.vehicles[0].registration, "XYZ789");
}
