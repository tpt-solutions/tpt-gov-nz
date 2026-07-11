//! Unit tests for the Fire and Emergency New Zealand ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawFenzBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/fenz_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawFenzBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.fenz_id, "FENZ-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert!(t.fire_safety.is_some());
    assert_eq!(t.fire_safety.as_ref().unwrap().property, "12 Totara Street, Porirua");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.incidents.len(), 1);
    assert_eq!(t.incidents[0].reference, "FENZ-2026-050");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawFenzBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "fenz_id": "FENZ-100001", "did": "", "fire_safety": null, "incidents": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
