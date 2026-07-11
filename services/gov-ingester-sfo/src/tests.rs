//! Unit tests for the Serious Fraud Office ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawSfoBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/sfo_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawSfoBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.sfo_id, "SFO-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.investigations.len(), 1);
    assert_eq!(t.investigations[0].reference, "SFO-2026-014");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.outcomes.len(), 1);
    assert_eq!(t.outcomes[0].reference, "SFO-2025-009");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawSfoBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "sfo_id": "SFO-100001", "did": "", "investigations": [], "outcomes": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
