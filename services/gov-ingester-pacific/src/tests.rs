//! Unit tests for the Ministry for Pacific Peoples ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawPacificBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/pacific_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawPacificBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.pacific_id, "PAC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.programmes.len(), 1);
    assert_eq!(t.programmes[0].programme_name, "Tokelau Language Week");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.language_services.len(), 1);
    assert_eq!(t.language_services[0].service, "Gagana Samoa classes");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawPacificBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "pacific_id": "PAC-100001", "did": "", "programmes": [], "language_services": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
