//! Unit tests for the New Zealand Defence Force ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawNzdfBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/nzdf_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawNzdfBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.nzdf_id, "NZDF-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.service_records.len(), 1);
    assert_eq!(t.service_records[0].service_no, "NZDF-55821");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.deployments.len(), 1);
    assert_eq!(t.deployments[0].operation, "Burnham readiness");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawNzdfBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "nzdf_id": "NZDF-100001", "did": "", "service_records": [], "deployments": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
