//! Unit tests for the Maritime New Zealand ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawMaritimeBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/maritime_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMaritimeBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.maritime_id, "MAR-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.vessels.len(), 1);
    assert_eq!(t.vessels[0].vessel_name, "MV Tane Moana");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.incidents.len(), 1);
    assert_eq!(t.incidents[0].reference, "MAR-2026-02");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMaritimeBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "maritime_id": "MAR-100001", "did": "", "vessels": [], "incidents": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
