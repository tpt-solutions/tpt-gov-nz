//! Unit tests for the Earthquake Commission (Toka Tū Ake) ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawEqcBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/eqc_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawEqcBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.eqc_id, "EQC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.claims.len(), 1);
    assert_eq!(t.claims[0].reference, "EQC-2026-007");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert!(t.cover.is_some());
    assert_eq!(t.cover.as_ref().unwrap().property, "12 Totara Street, Porirua");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawEqcBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "eqc_id": "EQC-100001", "did": "", "claims": [], "cover": null }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
