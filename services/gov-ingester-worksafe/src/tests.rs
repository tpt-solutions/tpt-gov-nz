//! Unit tests for the WorkSafe New Zealand ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawWorksafeBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/worksafe_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawWorksafeBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.worksafe_id, "WS-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.inspections.len(), 1);
    assert_eq!(t.inspections[0].reference, "WS-I-2026-003");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.investigations.len(), 1);
    assert_eq!(t.investigations[0].reference, "WS-INV-2026-011");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawWorksafeBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "worksafe_id": "WS-100001", "did": "", "inspections": [], "investigations": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
