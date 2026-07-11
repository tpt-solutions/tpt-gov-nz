//! Unit tests for the Department of the Prime Minister and Cabinet ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawDpmcBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/dpmc_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawDpmcBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.dpmc_id, "DPMC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.honours.len(), 1);
    assert_eq!(t.honours[0].award_year, 2025);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.engagements.len(), 1);
    assert_eq!(t.engagements[0].event_name, "Citizens' Honours Reception");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawDpmcBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "dpmc_id": "DPMC-100001", "did": "", "honours": [], "engagements": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
