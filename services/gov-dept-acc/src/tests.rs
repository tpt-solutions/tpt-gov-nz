//! Integration + unit tests for the ACC department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://acc:acc_dev_password@localhost:5432/acc cargo test -p gov-dept-acc
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, AccError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.client_number, "ACC-100001");
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
    assert_eq!(data["clientNumber"], "ACC-100001");
    assert!(data["claims"].is_null());
}

#[sqlx::test]
async fn fetch_data_claims_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["acc:claims"]).await;
    let claims = data["claims"].as_array().unwrap();
    assert_eq!(claims.len(), 1);
    assert_eq!(claims[0]["claimNumber"], "ACC-5001");
    assert_eq!(claims[0]["claimType"], "work");
    assert_eq!(claims[0]["status"], "open");
    assert_eq!(claims[0]["weeklyCompensation"], 420);
}

#[sqlx::test]
async fn fetch_data_entitlements_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["acc:entitlements"]).await;
    assert_eq!(data["entitlements"]["hasEntitlement"], true);
    assert_eq!(data["entitlements"]["weeklyAmount"], 420);
    assert_eq!(data["entitlements"]["remainingWeeks"], 18);
}

#[sqlx::test]
async fn fetch_data_rehabilitation_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["acc:rehabilitation"]).await;
    let plans = data["rehabilitation"].as_array().unwrap();
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["planId"], "PLAN-1");
    assert_eq!(plans[0]["status"], "active");
}

// ── actions ──────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_lodge_claim_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "lodge-claim",
        &json!({ "claimType": "work", "injuryDate": "2025-06-01", "description": "Knee injury" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_lodge_claim_rejects_empty_description(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "lodge-claim",
        &json!({ "claimType": "work", "injuryDate": "2025-06-01", "description": "   " }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, AccError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_lodge_claim_rejects_bad_type(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "lodge-claim",
        &json!({ "claimType": "rocket", "injuryDate": "2025-06-01", "description": "Hurt" }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, AccError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_request_rehabilitation_review_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-rehabilitation-review",
        &json!({ "claimNumber": "ACC-5001" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
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
    assert!(matches!(err, AccError::InvalidAction(_)));
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
            json!({ "did": TEST_DID, "scopes": ["acc:claims", "acc:entitlements", "acc:rehabilitation"] })
                .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["clientNumber"], "ACC-100001");
    assert_eq!(body["claims"][0]["claimNumber"], "ACC-5001");
    assert_eq!(body["entitlements"]["hasEntitlement"], true);
    assert_eq!(body["rehabilitation"][0]["planId"], "PLAN-1");
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_acc_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "acc",
        vec!["acc:claims".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["acc:claims"],
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
        "scopes": ["acc:claims"],
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
