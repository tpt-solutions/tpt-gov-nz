//! Integration + unit tests for the MPI department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://mpi:mpi_dev_password@localhost:5432/mpi cargo test -p gov-dept-mpi
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, MpiError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.mpi_id, "MPI-100001");
}

#[sqlx::test]
async fn resolve_not_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, "did:gov:nz:missing")
        .await
        .unwrap();
    assert!(row.is_none());
}

async fn fetch_data(pool: &PgPool, scopes: &[&str]) -> Value {
    let app = build_app(pool.clone());
    let body = json!({ "did": TEST_DID, "scopes": scopes }).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[sqlx::test]
async fn fetch_data_no_scopes_returns_minimal(pool: PgPool) {
    let data = fetch_data(&pool, &[]).await;
    assert_eq!(data["mpiId"], "MPI-100001");
    assert!(data["registrations"].is_null());
}

#[sqlx::test]
async fn fetch_data_registrations_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["mpi:registrations"]).await;
    let registrations = data["registrations"].as_array().unwrap();
    assert_eq!(registrations.len(), 1);
    assert_eq!(registrations[0]["nzbn"], "9429046000000");
    assert_eq!(registrations[0]["status"], "registered");
}

#[sqlx::test]
async fn fetch_data_certifications_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["mpi:certifications"]).await;
    let certifications = data["certifications"].as_array().unwrap();
    assert_eq!(certifications.len(), 1);
    assert_eq!(certifications[0]["certNumber"], "MPI-CERT-2026-001");
    assert_eq!(certifications[0]["category"], "export-certificate");
}

// ── actions ──────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_apply_export_certificate_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "apply-export-certificate",
        &json!({ "product": "Apples" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_apply_export_certificate_rejects_empty_product(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "apply-export-certificate",
        &json!({ "product": "  " }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, MpiError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_unknown_is_invalid(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "frobnicate",
        &json!({}),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, MpiError::InvalidAction(_)));
}

// ── full HTTP round-trip ─────────────────────────────────────────────────────────

#[sqlx::test]
async fn http_round_trip(pool: PgPool) {
    let app = build_app(pool);

    let resp = app
        .clone()
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let data = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "did": TEST_DID, "scopes": ["mpi:registrations", "mpi:certifications"] })
                .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["mpiId"], "MPI-100001");
    assert_eq!(body["registrations"][0]["nzbn"], "9429046000000");
    assert_eq!(body["certifications"][0]["certNumber"], "MPI-CERT-2026-001");
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_mpi_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "mpi",
        vec!["mpi:registrations".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["mpi:registrations"],
        "requesting_dept_id": "ird",
        "consent_grants": [grant],
    })
    .to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test]
async fn cross_dept_request_without_grant_is_forbidden(pool: PgPool) {
    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["mpi:registrations"],
        "requesting_dept_id": "ird",
        "consent_grants": [],
    })
    .to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

// ── helpers ────────────────────────────────────────────────────────────────────

async fn citizen_id(pool: &PgPool) -> uuid::Uuid {
    crate::db::resolve_by_did(pool, TEST_DID)
        .await
        .unwrap()
        .expect("test citizen seeded")
        .id
}
