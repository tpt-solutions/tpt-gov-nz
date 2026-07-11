//! Unit tests for the Te Kawa Mataaho Public Service Commission ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawPublicserviceBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/publicservice_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawPublicserviceBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.psc_id, "PSC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.workforce.len(), 1);
    assert_eq!(t.workforce[0].report_year, 2025);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.agency_ratings.len(), 1);
    assert_eq!(t.agency_ratings[0].agency, "Department of Internal Affairs");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawPublicserviceBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "psc_id": "PSC-100001", "did": "", "workforce": [], "agency_ratings": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
