//! Unit tests for the Ministry for Regulation ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawRegulationBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/regulation_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawRegulationBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.regulation_id, "REG-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.regulatory_reviews.len(), 1);
    assert_eq!(t.regulatory_reviews[0].topic, "Building consenting");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.proposals.len(), 1);
    assert_eq!(t.proposals[0].title, "Reduce duplicate reporting");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawRegulationBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "regulation_id": "REG-100001", "did": "", "regulatory_reviews": [], "proposals": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
