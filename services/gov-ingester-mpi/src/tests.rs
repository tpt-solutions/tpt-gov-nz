//! Unit tests for the MPI ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-mpi --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawMpiBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mpi_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMpiBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.mpi_id, "MPI-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.registrations.len(), 1);
    assert_eq!(t.registrations[0].nzbn, "9429046000000");
    assert_eq!(t.certifications.len(), 1);
    assert_eq!(t.certifications[0].cert_number, "MPI-CERT-2026-001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.registrations.len(), 0);
    assert_eq!(t2.certifications.len(), 1);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMpiBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "mpiId": "MPI-9", "did": "", "registrations": [], "certifications": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_registered_date_is_rejected() {
    let raw: RawMpiBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "mpiId": "MPI-9", "did": "did:gov:nz:t", "registrations": [{ "nzbn": "N1", "businessName": "B", "type": "food-business", "status": "registered", "registeredDate": "nope" }], "certifications": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawMpiBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "mpiId": "MPI-9", "did": "did:gov:nz:t", "registrations": [], "certifications": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.registrations.len(), 0);
    assert_eq!(t.certifications.len(), 0);
}
