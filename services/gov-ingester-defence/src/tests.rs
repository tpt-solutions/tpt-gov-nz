//! Unit tests for the Ministry of Defence ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawDefenceBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/defence_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawDefenceBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.defence_id, "DEF-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.procurements.len(), 1);
    assert_eq!(t.procurements[0].programme, "Frigate sustainment");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.bases.len(), 1);
    assert_eq!(t.bases[0].name, "Trentham Military Camp");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawDefenceBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "defence_id": "DEF-100001", "did": "", "procurements": [], "bases": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
