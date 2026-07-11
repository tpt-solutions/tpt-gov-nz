//! Unit tests for the Statistics New Zealand ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-statsnz --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawStatsnzBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/statsnz_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawStatsnzBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.stats_id, "STATS-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.census.len(), 1);
    assert_eq!(t.census[0].census_year, 2023);
    assert_eq!(t.census[0].region, "Auckland");
    let p = t.profile.expect("profile present");
    assert_eq!(p.record_count, 1);

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.census.len(), 0);
    assert!(t2.profile.is_some());
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawStatsnzBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "statsId": "STATS-9", "did": "", "census": [], "profile": null }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_profile_date_is_rejected() {
    let raw: RawStatsnzBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "statsId": "STATS-9", "did": "did:gov:nz:t", "census": [], "profile": { "dataSummary": "x", "recordCount": 0, "lastUpdated": "nope" } }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawStatsnzBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "statsId": "STATS-9", "did": "did:gov:nz:t", "census": [], "profile": null }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.census.len(), 0);
    assert!(t.profile.is_none());
}
