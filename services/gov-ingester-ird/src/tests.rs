//! Integration + unit tests for the IRD ingester.
//!
//! Run with a Postgres available and pointing `DATABASE_URL` at the IRD dept DB
//! (migrations are applied by `sqlx::test` from the shared dept migrations):
//!
//! ```sh
//! DATABASE_URL=postgresql://ird:ird_dev_password@localhost:5432/ird \
//!   cargo test -p gov-ingester-ird
//! ```

use std::path::PathBuf;

use crate::{
    db,
    ingest,
    models::IngestionStatus,
    raw::RawIrdBatch,
    transform::transform_citizen,
    transport::mock::MockTransport,
};

#[sqlx::test(migrations = "../gov-dept-ird/migrations")]
async fn upsert_citizen_is_idempotent(pool: sqlx::PgPool) {
    let (_, first) = db::upsert_citizen(&pool, "did:gov:nz:t", "111-222-333")
        .await
        .unwrap();
    assert!(first, "first upsert should insert");

    let (_, second) = db::upsert_citizen(&pool, "did:gov:nz:t", "111-222-333")
        .await
        .unwrap();
    assert!(!second, "second upsert should update, not insert");

    // Re-run is safe: still an update, no duplicate row.
    let (_, third) = db::upsert_citizen(&pool, "did:gov:nz:t", "111-222-333")
        .await
        .unwrap();
    assert!(!third);
}

#[sqlx::test(migrations = "../gov-dept-ird/migrations")]
async fn upsert_tax_assessment_is_idempotent(pool: sqlx::PgPool) {
    let (cid, _) = db::upsert_citizen(&pool, "did:gov:nz:tax", "222-333-444")
        .await
        .unwrap();

    let e = crate::models::TaxEntity {
        assessment_year: 2025,
        tax_code: "M".into(),
        total_income: sqlx::types::Decimal::from(1000),
        taxable_income: sqlx::types::Decimal::from(1000),
        tax_liability: sqlx::types::Decimal::from(100),
        tax_paid: sqlx::types::Decimal::from(100),
        tax_refund_due: sqlx::types::Decimal::from(0),
        tax_owing: sqlx::types::Decimal::from(0),
        assessment_status: "final".into(),
    };
    assert!(db::upsert_tax_assessment(&pool, cid, &e).await.unwrap());
    assert!(!db::upsert_tax_assessment(&pool, cid, &e).await.unwrap());
}

// ── full run with mock transport ────────────────────────────────────────────────

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/ird_batch.json")
}

#[sqlx::test(migrations = "../gov-dept-ird/migrations")]
async fn run_once_with_mock_transport(pool: sqlx::PgPool) {
    let transport = MockTransport::new(fixture_path());
    let summary = ingest::run_once(&pool, &transport).await.unwrap();

    assert_eq!(summary.citizens_processed, 2);
    assert_eq!(summary.status, IngestionStatus::Success);
    // Each citizen maps to 6 entity rows. test-citizen-001 is seeded by migration 008
    // (so all 6 are updates); test-citizen-002 is new (so all 6 are inserts).
    assert_eq!(summary.rows_inserted, 6);
    assert_eq!(summary.rows_updated, 6);
    assert!(summary.batch_id.as_deref() == Some("IRD-2025-07-B001"));

    // Audit row was written.
    let run = db::latest_run(&pool).await.unwrap().expect("audit row");
    assert_eq!(run.0, "mock");
    assert_eq!(run.1, 2);
    assert_eq!(run.3, "success");

    // Data landed in the dept tables.
    let citizen = db::upsert_citizen(&pool, "did:gov:nz:test-citizen-002", "987-654-321")
        .await
        .unwrap();
    assert!(!citizen.1, "already upserted by the run");

    let ks = sqlx::query_scalar!(
        "SELECT contribution_rate FROM kiwisaver_memberships WHERE citizen_id = $1",
        citizen.0
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(ks, sqlx::types::Decimal::from(4));
}

#[sqlx::test(migrations = "../gov-dept-ird/migrations")]
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
    let batch: RawIrdBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);
    let t = transform_citizen(&batch.citizens[1]).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-002");
    assert!(t.gst.registered);
    assert_eq!(t.gst.gst_number.as_deref(), Some("098-765-432"));
}
