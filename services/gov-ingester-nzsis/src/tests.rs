//! Unit tests for the New Zealand Security Intelligence Service ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawNzsisBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/nzsis_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawNzsisBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.nzsis_id, "NZSIS-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.mandates.len(), 1);
    assert_eq!(t.mandates[0].reference, "NZSIS-M-2026-002");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.threats.len(), 1);
    assert_eq!(t.threats[0].reference, "NZSIS-T-2026-014");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawNzsisBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "nzsis_id": "NZSIS-100001", "did": "", "mandates": [], "threats": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
