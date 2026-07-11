//! Integration + unit tests for the Corrections department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://corrections:corrections_dev_password@localhost:5432/corrections cargo test -p gov-dept-corrections
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, CorrectionsError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.corrections_id, "COR-100001");
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
    assert_eq!(data["correctionsId"], "COR-100001");
    assert!(data["probation"].is_null());
    assert!(data["case"].is_null());
}

#[sqlx::test]
async fn fetch_data_probation_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["corrections:probation"]).await;
    let probation = data["probation"].as_object().unwrap();
    assert_eq!(probation["status"], "active");
    assert_eq!(probation["officerName"], "Officer R. Reedy");
    assert_eq!(probation["location"], "Auckland Probation Hub");
}

#[sqlx::test]
async fn fetch_data_case_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["corrections:case"]).await;
    let cases = data["case"].as_array().unwrap();
    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0]["caseNumber"], "COR-C0001");
    assert_eq!(cases[0]["sentenceType"], "supervision");
    assert_eq!(cases[0]["endDate"], "2027-03-01");
}

// ── actions ────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_request_sentence_summary_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-sentence-summary",
        &json!({ "purpose": "Court appearance preparation" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_request_sentence_summary_rejects_empty_purpose(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-sentence-summary",
        &json!({ "purpose": "   " }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, CorrectionsError::InvalidAction(_)));
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
    assert!(matches!(err, CorrectionsError::InvalidAction(_)));
}

// ── full HTTP round-trip ───────────────────────────────────────────────────────

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
            json!({ "did": TEST_DID, "scopes": ["corrections:probation", "corrections:case"] })
                .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["correctionsId"], "COR-100001");
    assert_eq!(body["probation"]["status"], "active");
    assert_eq!(body["case"][0]["caseNumber"], "COR-C0001");
}

// ── cross-department federation (consent-gated) ───────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_corrections_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "corrections",
        vec!["corrections:probation".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["corrections:probation"],
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
        "scopes": ["corrections:probation"],
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
