//! Unit tests for the The Treasury ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawTreasuryBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/treasury_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawTreasuryBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.treasury_id, "TRE-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.budget.len(), 1);
    assert_eq!(t.budget[0].fiscal_year, 2026);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert!(t.economic_outlook.is_some());
    assert_eq!(t.economic_outlook.as_ref().unwrap().forecast_year, 2026);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawTreasuryBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "treasury_id": "TRE-100001", "did": "", "budget": [], "economic_outlook": null }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
