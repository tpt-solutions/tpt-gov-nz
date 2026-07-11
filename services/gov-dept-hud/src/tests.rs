//! Integration + unit tests for the HUD department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://hud:hud_dev_password@localhost:5432/hud cargo test -p gov-dept-hud
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, HudError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.client_number, "HUD-100001");
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
    assert_eq!(data["clientNumber"], "HUD-100001");
    assert!(data["applications"].is_null());
}

#[sqlx::test]
async fn fetch_data_applications_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["hud:applications"]).await;
    let applications = data["applications"].as_array().unwrap();
    assert_eq!(applications.len(), 1);
    assert_eq!(applications[0]["applicationNumber"], "HUD-A5001");
    assert_eq!(applications[0]["status"], "waitlisted");
    assert_eq!(applications[0]["priorityBand"], "B");
}

#[sqlx::test]
async fn fetch_data_tenancy_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["hud:tenancy"]).await;
    let tenancies = data["tenancies"].as_array().unwrap();
    assert_eq!(tenancies.len(), 1);
    assert_eq!(tenancies[0]["tenancyId"], "HUD-TEN-1");
    assert_eq!(tenancies[0]["incomeRelatedRent"], true);
}

#[sqlx::test]
async fn fetch_data_maintenance_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["hud:maintenance"]).await;
    let requests = data["maintenanceRequests"].as_array().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0]["requestNumber"], "HUD-M3001");
    assert_eq!(requests[0]["status"], "scheduled");
}

// ── actions ──────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_submit_housing_application_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "submit-housing-application",
        &json!({ "applicationType": "emergency-housing", "bedroomsNeeded": 1, "reason": "Currently in a motel" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_submit_housing_application_rejects_bad_type(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "submit-housing-application",
        &json!({ "applicationType": "castle", "reason": "x" }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, HudError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_request_maintenance_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-maintenance",
        &json!({ "category": "heating", "description": "Heat pump not working" }),
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
    assert!(matches!(err, HudError::InvalidAction(_)));
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
            json!({ "did": TEST_DID, "scopes": ["hud:applications", "hud:tenancy", "hud:maintenance"] })
                .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["clientNumber"], "HUD-100001");
    assert_eq!(body["applications"][0]["applicationNumber"], "HUD-A5001");
    assert_eq!(body["tenancies"][0]["tenancyId"], "HUD-TEN-1");
    assert_eq!(body["maintenanceRequests"][0]["requestNumber"], "HUD-M3001");
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_hud_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "hud",
        vec!["hud:applications".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["hud:applications"],
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
        "scopes": ["hud:applications"],
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
