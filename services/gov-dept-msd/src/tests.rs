//! Integration + unit tests for the MSD department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://msd:msd_dev_password@localhost:5432/msd cargo test -p gov-dept-msd
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, MsdError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.client_number, "MSD-100001");
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
    assert_eq!(data["clientNumber"], "MSD-100001");
    assert!(data["studylink"].is_null());
    assert!(data["caseHistory"].is_null());
}

#[sqlx::test]
async fn fetch_data_studylink_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["msd:studylink"]).await;
    let sl = data["studylink"].as_object().unwrap();
    assert_eq!(sl["hasStudentLoan"], true);
    assert!(sl["loanBalance"].is_number());
    assert_eq!(sl["repaymentPlan"], "standard");
    assert_eq!(sl["hasAllowance"], true);
    assert_eq!(sl["allowanceType"], "living-allowance");
    assert_eq!(sl["nextPaymentDate"], "2026-07-15");
    assert!(sl["weeklyAmount"].is_number());
}

#[sqlx::test]
async fn fetch_data_case_history_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["msd:case-history"]).await;
    let history = data["caseHistory"].as_array().unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0]["eventDate"], "2026-06-10");
    assert_eq!(history[0]["serviceLine"], "StudyLink");
    assert_eq!(history[1]["eventId"], "MSD-EVT-001");
}

// ── actions ──────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_apply_student_allowance_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "apply-student-allowance",
        &json!({ "courseOfStudy": "Bachelor of Science", "provider": "University of Auckland" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_apply_student_allowance_rejects_missing_provider(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "apply-student-allowance",
        &json!({ "courseOfStudy": "BSc", "provider": "  " }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, MsdError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_update_loan_repayment_plan_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "update-loan-repayment-plan",
        &json!({ "plan": "income-indexed" }),
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
    assert!(matches!(err, MsdError::InvalidAction(_)));
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
            json!({ "did": TEST_DID, "scopes": ["msd:studylink", "msd:case-history"] })
                .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["clientNumber"], "MSD-100001");
    assert_eq!(body["studylink"]["hasStudentLoan"], true);
    assert_eq!(body["caseHistory"][0]["eventId"], "MSD-EVT-002");
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_msd_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "msd",
        vec!["msd:studylink".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["msd:studylink"],
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
        "scopes": ["msd:studylink"],
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
