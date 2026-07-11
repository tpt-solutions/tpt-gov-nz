// @ts-nocheck
// Emitter for Phase 3 departments. Imported by gen-phase3-run.mjs (or run directly).
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { DEPTS, T, pascal, camel, upper, sqlLit, W, ROOT } from "./gen-phase3-dept.mjs";

const R = (rel) => readFileSync(join(ROOT, rel), "utf8");

// ── small string helpers ───────────────────────────────────────────────────────
const comma = (arr, sep = ", ") => arr.join(sep);
const indent = (s, n = 4) => s.split("\n").map((l) => " ".repeat(n) + l).join("\n");

// Field list strings for a given entity
const fieldNames = (e) => e.fields.map((f) => f.n);
const fieldList = (e, sep = ", ") => e.fields.map((f) => f.n).join(sep);
const fieldTypes = (e, sep = ", ") => e.fields.map((f) => `${f.n}: ${T[f.t].rust}`).join(sep);
const fieldSelect = (e) => "id, " + fieldList(e);

// ── Rust dept service ───────────────────────────────────────────────────────────
function genDeptService(d) {
  const id = d.id;
  const P = pascal(id);
  const U = upper(id);
  const local = d.localId.field;
  const localEx = d.localId.example;
  const svc = `services/gov-dept-${id}`;

  // migrations
  const mig = {};
  mig["001_citizens.sql"] = `CREATE TABLE IF NOT EXISTS citizens (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did           TEXT UNIQUE NOT NULL,
    ${local}      TEXT UNIQUE NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_citizens_did ON citizens (did);
`;

  let mnum = 2;
  const entityMig = {};
  for (const e of d.entities) {
    const tbl = `${id}_${e.key}`;
    const biz = e.cardinality === "many" ? `UNIQUE (citizen_id, ${e.fields[0].n})` : `UNIQUE (citizen_id)`;
    entityMig[e.key] = `CREATE TABLE IF NOT EXISTS ${tbl} (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    ${e.fields.map((f) => `${f.n.padEnd(16)} ${T[f.t].pg} NOT NULL,`).join("\n    ")}
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ${biz}
);

CREATE INDEX idx_${tbl}_citizen ON ${tbl} (citizen_id);
`;
    mnum++;
  }

  mig["090_actions_log.sql"] = `-- Immutable audit log of all actions taken against ${d.name} data
CREATE TABLE IF NOT EXISTS actions_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id),
    action_type     TEXT NOT NULL,
    parameters      JSONB NOT NULL DEFAULT '{}',
    performed_by    TEXT NOT NULL,
    ai_level        TEXT,
    result_success  BOOLEAN NOT NULL,
    result_message  TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE RULE no_update_actions_log AS ON UPDATE TO actions_log DO INSTEAD NOTHING;
CREATE RULE no_delete_actions_log AS ON DELETE TO actions_log DO INSTEAD NOTHING;
`;

  mig["091_ingestion_runs.sql"] = `-- Ingestion audit log: records every batch the ingester pulls from the legacy system.
CREATE TABLE IF NOT EXISTS ingestion_runs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source              TEXT NOT NULL,
    batch_id            TEXT,
    run_started_at      TIMESTAMPTZ NOT NULL,
    run_finished_at     TIMESTAMPTZ,
    citizens_processed  INTEGER NOT NULL DEFAULT 0,
    rows_inserted       INTEGER NOT NULL DEFAULT 0,
    rows_updated        INTEGER NOT NULL DEFAULT 0,
    status              TEXT NOT NULL DEFAULT 'running'
                        CHECK (status IN ('running', 'success', 'failed')),
    error_message       TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ingestion_runs_started ON ingestion_runs (run_started_at DESC);
`;

  // seed
  const ex2 = localEx.replace(/001$/, "002");
  let seed = `INSERT INTO citizens (did, ${local})
VALUES
    ('did:gov:nz:test-citizen-001', '${localEx}'),
    ('did:gov:nz:test-citizen-002', '${ex2}')
ON CONFLICT DO NOTHING;
`;
  for (const e of d.entities) {
    const tbl = `${id}_${e.key}`;
    const vals = e.fields.map((f) => sqlLit(e.sample[f.j], f.t)).join(", ");
    seed += `
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO ${tbl} (citizen_id, ${fieldList(e)}) SELECT c.id, ${vals} FROM c
ON CONFLICT DO NOTHING;
`;
  }
  mig["092_seed_dev.sql"] = `-- Development seed data — test citizens. Only runs in dev/demo environments.\n` + seed;

  // idempotency constraints
  let idem = "";
  for (const e of d.entities) {
    const tbl = `${id}_${e.key}`;
    const biz = e.cardinality === "many" ? `citizen_id, ${e.fields[0].n}` : `citizen_id`;
    idem += `ALTER TABLE ${tbl}
    ADD CONSTRAINT ${tbl}_business_key UNIQUE (${biz});\n\n`;
  }
  mig["093_ingester_idempotency.sql"] = `-- Idempotency keys for the ingester.\n` + idem;

  for (const [name, content] of Object.entries(mig)) {
    W(`${svc}/migrations/${name}`, content);
  }

  // main.rs
  W(`${svc}/src/main.rs`, `use std::net::SocketAddr;
use tracing::info;

use gov_dept_${id}::build_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "gov_dept_${id}=info,tower_http=debug".into()),
        )
        .json()
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let http_listen: SocketAddr = std::env::var("TPT__GOV__HTTP_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:${d.port}".into())
        .parse()?;

    let pool = sqlx::PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    let app = build_app(pool);

    info!(listen = %http_listen, dept = "${id}", "gov-dept-${id} starting");
    let listener = tokio::net::TcpListener::bind(http_listen).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
`);

  // lib.rs
  W(`${svc}/src/lib.rs`, `use axum::{routing::get, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

mod actions;
mod consent;
mod db;
mod error;
mod opa;
mod routes;

pub use error::${P}Error;

/// Build the ${d.name} department service router with the given connection pool as state.
pub fn build_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::health))
        .route("/citizen/resolve", axum::routing::post(routes::resolve_citizen))
        .route("/citizen/data", axum::routing::post(routes::fetch_data))
        .route("/citizen/action", axum::routing::post(routes::submit_action))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests;
`);

  // error.rs
  W(`${svc}/src/error.rs`, `use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ${P}Error {
    #[error("Citizen not found")]
    CitizenNotFound,

    #[error("Scope not granted: {0}")]
    ScopeNotGranted(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid action: {0}")]
    InvalidAction(String),

    #[error("Action failed: {0}")]
    ActionFailed(String),
}

impl IntoResponse for ${P}Error {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ${P}Error::CitizenNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ${P}Error::ScopeNotGranted(_) => (StatusCode::FORBIDDEN, self.to_string()),
            ${P}Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_owned()),
            ${P}Error::InvalidAction(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ${P}Error::ActionFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
`);

  // db.rs
  let rows = d.entities.map((e) => {
    const EP = pascal(e.key);
    const tbl = `${id}_${e.key}`;
    const fetchSig = e.cardinality === "many"
      ? `pub async fn fetch_${e.key}(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<${EP}Row>> {
    sqlx::query_as!(
        ${EP}Row,
        "SELECT ${fieldSelect(e)} FROM ${tbl} WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}`
      : `pub async fn fetch_${e.key}(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<${EP}Row>> {
    sqlx::query_as!(
        ${EP}Row,
        "SELECT ${fieldSelect(e)} FROM ${tbl} WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}`;
    return `#[derive(Debug, sqlx::FromRow)]
pub struct ${EP}Row {
    pub id: Uuid,
    ${e.fields.map((f) => `pub ${f.n}: ${T[f.t].rust},`).join("\n    ")}
}

${fetchSig}`;
  }).join("\n\n");

  W(`${svc}/src/db.rs`, `use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub ${local}: String,
}

${rows}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, ${local} FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn log_action(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: serde_json::Value,
    performed_by: &str,
    ai_level: Option<&str>,
    result_success: bool,
    result_message: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO actions_log (citizen_id, action_type, parameters, performed_by, ai_level, result_success, result_message)
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        citizen_id,
        action_type,
        parameters,
        performed_by,
        ai_level,
        result_success,
        result_message
    )
    .execute(pool)
    .await?;
    Ok(())
}
`);

  // routes.rs
  let scopeBlocks = d.entities.map((e) => {
    const jsonFields = e.fields.map((f) => `"${f.j}": ${T[f.t].json(e.cardinality === "many" ? "c" : "p")},`).join("\n            ");
    if (e.cardinality === "many") {
      return `    let ${e.key} = if has_scope("${e.scope}") {
        Some(db::fetch_${e.key}(&pool, citizen.id).await?)
    } else {
        None
    };

    let ${e.key}_json = ${e.key}.as_ref().map(|rows| {
        json!(rows.iter().map(|c| json!({
            ${jsonFields}
        })).collect::<Vec<_>>())
    });`;
    }
    return `    let ${e.key} = if has_scope("${e.scope}") {
        Some(db::fetch_${e.key}(&pool, citizen.id).await?)
    } else {
        None
    };

    let ${e.key}_json = ${e.key}.as_ref().map(|p| {
        json!({
            ${jsonFields}
        })
    });`;
  }).join("\n\n");

  const respFields = d.entities.map((e) => `        "${e.key}": ${e.key}_json,`).join("\n");

  const actionMatch = d.actions.map((a) => `        "${a.type}" => ${camel(a.type)}(pool, citizen_id, parameters).await,`).join("\n");

  W(`${svc}/src/routes.rs`, `use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    actions,
    consent,
    db,
    error::${P}Error,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "${id}" }))
}

#[derive(Deserialize)]
pub struct ResolveRequest {
    pub did: String,
}

#[derive(Serialize)]
pub struct ResolveResponse {
    pub did: String,
    #[serde(rename = "deptLocalId")]
    pub dept_local_id: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

pub async fn resolve_citizen(
    State(pool): State<PgPool>,
    Json(req): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse>, ${P}Error> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(${P}Error::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.${local},
        display_name: None,
    }))
}

#[derive(Deserialize)]
pub struct DataRequest {
    pub did: String,
    pub scopes: Vec<String>,
    #[serde(default)]
    pub requesting_dept_id: Option<String>,
    #[serde(default)]
    pub consent_grants: Vec<gov_identity_core::DataGrantCredential>,
}

fn is_direct_access(requesting_dept_id: &Option<String>) -> bool {
    match requesting_dept_id.as_deref() {
        None | Some("citizen") | Some("staff") => true,
        Some(_) => false,
    }
}

pub async fn fetch_data(
    State(pool): State<PgPool>,
    Json(req): Json<DataRequest>,
) -> Result<Json<Value>, ${P}Error> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(${P}Error::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(${P}Error::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

${scopeBlocks}

    Ok(Json(json!({
        "${local}": citizen.${local},
${respFields}
    })))
}

#[derive(Deserialize)]
pub struct ActionRequest {
    pub did: String,
    pub r#type: String,
    pub parameters: Value,
    #[serde(default = "default_performed_by")]
    pub performed_by: String,
    pub ai_level: Option<String>,
}

fn default_performed_by() -> String {
    "citizen".to_owned()
}

pub async fn submit_action(
    State(pool): State<PgPool>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<Value>, ${P}Error> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(${P}Error::CitizenNotFound)?;

    actions::execute(
        &pool,
        citizen.id,
        &req.r#type,
        &req.parameters,
        &req.performed_by,
        req.ai_level.as_deref(),
    )
    .await
}
`);

  // actions.rs
  const handlers = d.actions.map((a) => {
    const h = camel(a.type);
    if (a.params && a.params.length > 0) {
      const p = a.params[0];
      return `async fn ${h}(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, ${P}Error> {
    let ${p.name} = parameters
        .get("${p.name}")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ${P}Error::InvalidAction("${p.name} must not be empty".into()))?;

    Ok(json!({
        "success": true,
        "message": format!("{}{}", "${a.message}", ${p.name}),
        "${p.name}": ${p.name},
    }))
}`;
    }
    return `async fn ${h}(
    _pool: &PgPool,
    _citizen_id: Uuid,
    _parameters: &Value,
) -> Result<Value, ${P}Error> {
    Ok(json!({
        "success": true,
        "message": "${a.message}",
    }))
}`;
  }).join("\n\n");

  W(`${svc}/src/actions.rs`, `use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::${P}Error};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, ${P}Error> {
    let result = match action_type {
${actionMatch}
        _ => Err(${P}Error::InvalidAction(format!(
            "Unknown action: {action_type}"
        ))),
    };

    let (success, message) = match &result {
        Ok(_) => (true, None),
        Err(e) => (false, Some(e.to_string())),
    };

    db::log_action(
        pool,
        citizen_id,
        action_type,
        parameters.clone(),
        performed_by,
        ai_level,
        success,
        message.as_deref(),
    )
    .await?;

    result.map(Json)
}

${handlers}
`);

  // consent.rs
  const firstScope = d.entities[0].scope;
  W(`${svc}/src/consent.rs`, `//! Cross-department consent verification for the ${d.name} department node.
//!
//! Mirrors \`policies/${id}.rego\` so the same allow/deny decision can be made locally
//! (and in unit tests) without a running OPA sidecar.

use crate::error::${P}Error;
use ed25519_dalek::VerifyingKey;
use gov_identity_core::DataGrantCredential;

pub struct ConsentInput<'a> {
    pub requesting_dept_id: &'a str,
    pub citizen_did: &'a str,
    pub requested_scopes: &'a [String],
    pub consent_grants: &'a [DataGrantCredential],
}

pub fn evaluate_allow(input: &ConsentInput) -> bool {
    input
        .requested_scopes
        .iter()
        .all(|scope| scope_covered(input, scope))
}

fn scope_covered(input: &ConsentInput, scope: &str) -> bool {
    let now = chrono::Utc::now().timestamp();
    input.consent_grants.iter().any(|grant| {
        grant.requesting_dept_id == input.requesting_dept_id
            && grant.providing_dept_id == "${id}"
            && grant.citizen_did.as_str() == input.citizen_did
            && grant.expires_at > now
            && grant.scopes.iter().any(|s| s == scope)
    })
}

pub fn denied_scopes(input: &ConsentInput) -> Vec<String> {
    input
        .requested_scopes
        .iter()
        .filter(|scope| !scope_covered(input, scope))
        .cloned()
        .collect()
}

pub fn verify_access(
    input: &ConsentInput,
    issuer_key: Option<&VerifyingKey>,
) -> Result<(), ${P}Error> {
    if !evaluate_allow(input) {
        return Err(${P}Error::ScopeNotGranted(denied_scopes(input).join(",")));
    }

    if let Some(key) = issuer_key {
        let all_signed = input.requested_scopes.iter().all(|scope| {
            input.consent_grants.iter().any(|grant| {
                grant.scopes.iter().any(|s| s == scope)
                    && grant.requesting_dept_id == input.requesting_dept_id
                    && grant.providing_dept_id == "${id}"
                    && grant.citizen_did.as_str() == input.citizen_did
                    && grant.expires_at > chrono::Utc::now().timestamp()
                    && grant.verify_signature(key).is_ok()
            })
        });
        if !all_signed {
            return Err(${P}Error::ScopeNotGranted("grant signature invalid".into()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gov_identity_core::CredentialIssuer;

    fn issuer() -> CredentialIssuer {
        CredentialIssuer::generate()
    }

    fn grant(
        issuer: &CredentialIssuer,
        requesting: &str,
        scopes: Vec<String>,
    ) -> DataGrantCredential {
        issuer.issue_data_grant(
            gov_identity_core::GovDid::parse("did:gov:nz:test-citizen-001").unwrap(),
            requesting,
            "${id}",
            scopes,
            3600,
        )
    }

    #[test]
    fn ${id}_grant_allows_${id}_scope() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["${firstScope}".into()]);
        let scopes = vec!["${firstScope}".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(evaluate_allow(&input));
        assert!(verify_access(&input, Some(&i.verifying_key())).is_ok());
    }

    #[test]
    fn wrong_dept_grant_denies() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["ird:income".into()]);
        let scopes = vec!["${firstScope}".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }

    #[test]
    fn expired_grant_is_denied() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["${firstScope}".into()]);
        let mut g2 = g;
        g2.expires_at = chrono::Utc::now().timestamp() - 10;
        let scopes = vec!["${firstScope}".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g2],
        };
        assert!(!evaluate_allow(&input));
    }
}
`);

  // opa.rs
  W(`${svc}/src/opa.rs`, `//! Optional OPA sidecar client for the ${d.name} node.

use crate::consent::ConsentInput;
use gov_identity_core::DataGrantCredential;
use serde::Serialize;

#[cfg(feature = "opa")]
#[derive(Serialize)]
struct OpaInput<'a> {
    requesting_dept_id: &'a str,
    providing_dept_id: &'static str,
    citizen_did: &'a str,
    requested_scopes: &'a [String],
    consent_grants: &'a [DataGrantCredential],
}

#[cfg(feature = "opa")]
#[derive(serde::Deserialize)]
struct OpaResponse {
    result: OpaAllow,
}

#[cfg(feature = "opa")]
#[derive(serde::Deserialize)]
struct OpaAllow {
    allow: bool,
}

#[cfg(feature = "opa")]
pub async fn evaluate(input: &ConsentInput<'_>) -> Option<bool> {
    let base = std::env::var("OPA_URL").ok()?;
    let client = reqwest::Client::new();

    let body = OpaInput {
        requesting_dept_id: input.requesting_dept_id,
        providing_dept_id: "${id}",
        citizen_did: input.citizen_did,
        requested_scopes: input.requested_scopes,
        consent_grants: input.consent_grants,
    };

    let url = format!("{base}/v1/data/${id}/consent");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({ "input": body }))
        .send()
        .await
        .ok()?;

    let parsed: OpaResponse = resp.json().await.ok()?;
    Some(parsed.result.allow)
}

#[cfg(not(feature = "opa"))]
pub async fn evaluate(_input: &ConsentInput<'_>) -> Option<bool> {
    None
}
`);

  // Dockerfile (dept)
  W(`${svc}/Dockerfile`, `FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-${id}/ services/gov-dept-${id}/

# Build only the ${d.name} service
RUN cargo build --release -p gov-dept-${id}

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-${id} /usr/local/bin/gov-dept-${id}
COPY --from=builder /app/services/gov-dept-${id}/migrations /migrations

EXPOSE ${d.port} ${d.fedPort}

CMD ["gov-dept-${id}"]
`);

  // Cargo.toml (dept)
  W(`${svc}/Cargo.toml`, `[package]
name = "gov-dept-${id}"
description = "${d.name} native service for tpt-gov-nz"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "gov-dept-${id}"
path = "src/main.rs"

[dependencies]
gov-federation-core = { path = "../../crates/gov-federation-core" }
gov-federation-node = { path = "../../crates/gov-federation-node" }
gov-identity-core = { path = "../../crates/gov-identity-core" }
tokio = { workspace = true }
axum = { workspace = true }
tower-http = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
sqlx = { workspace = true, features = ["macros", "migrate"] }
uuid = { workspace = true }
chrono = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
dotenvy = { workspace = true }
ed25519-dalek = { workspace = true }
reqwest = { version = "0.12", optional = true }

[features]
opa = ["dep:reqwest"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(feature, values("opa"))'] }

[dev-dependencies]
tokio = { workspace = true }
tower = { workspace = true }
http = "1"
hyper = { workspace = true }
uuid = { workspace = true }
`);

  // tests.rs
  const firstE = d.entities[0];
  const firstLit = lit(sampleVal(firstE, 0));

  const scopeTests = d.entities.map((e) => {
    const lit0 = lit(sampleVal(e, 0));
    if (e.cardinality === "many") {
      return `#[sqlx::test]
async fn fetch_data_${e.key}_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["${e.scope}"]).await;
    let arr = data["${e.key}"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["${e.fields[0].j}"], ${lit0});
}`;
    }
    return `#[sqlx::test]
async fn fetch_data_${e.key}_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["${e.scope}"]).await;
    let obj = data["${e.key}"].as_object().unwrap();
    assert_eq!(obj["${e.fields[0].j}"], ${lit0});
}`;
  }).join("\n\n");

  const firstAction = d.actions[0];
  const actionHasParam = firstAction.params && firstAction.params.length > 0;
  const validParams = actionHasParam ? `{ "${firstAction.params[0].name}": "Test value" }` : `{}`;
  const invalidParams = actionHasParam ? `{ "${firstAction.params[0].name}": "   " }` : `{}`;
  const invalidTest = actionHasParam ? `#[sqlx::test]
async fn action_${camel(firstAction.type)}_rejects_empty(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "${firstAction.type}",
        &json!(${invalidParams}),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, ${P}Error::InvalidAction(_)));
}` : "";

  W(`${svc}/src/tests.rs`, `//! Integration + unit tests for the ${d.name} department service.
//!
//! Run with a Postgres available:
//!
//! \`\`\`sh
//! DATABASE_URL=postgresql://${id}:${id}_dev_password@localhost:5432/${id} cargo test -p gov-dept-${id}
//! \`\`\`

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, ${P}Error};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.${local}, "${localEx}");
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
    assert_eq!(data["${local}"], "${localEx}");
    ${d.entities.map((e) => `assert!(data["${e.key}"].is_null());`).join("\n    ")}
}

${scopeTests}

// ── actions ───────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_${camel(firstAction.type)}_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "${firstAction.type}",
        &json!(${validParams}),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

${invalidTest}

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
    assert!(matches!(err, ${P}Error::InvalidAction(_)));
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
            json!({ "did": TEST_DID, "scopes": ${JSON.stringify(d.entities.map((e) => e.scope))} })
                .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["${local}"], "${localEx}");
    ${d.entities.map((e) => e.cardinality === "many"
      ? `assert_eq!(body["${e.key}"].as_array().unwrap().len(), 1);`
      : `assert!(body["${e.key}"].as_object().is_some());`).join("\n    ")}
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_${id}_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "${id}",
        vec!["${firstScope}".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["${firstScope}"],
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
        "scopes": ["${firstScope}"],
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
`);

  return { P, U, local, localEx };
}

function lit(v) {
  if (typeof v === "string") return `"${v}"`;
  if (typeof v === "boolean") return String(v);
  return String(v);
}

function sampleVal(e, idx) {
  return e.sample[e.fields[idx].j];
}

function genIngester(d) {
  const id = d.id;
  const P = pascal(id);
  const U = upper(id);
  const local = d.localId.field;
  const svc = `services/gov-ingester-${id}`;

  // Cargo.toml
  W(`${svc}/Cargo.toml`, `[package]
name = "gov-ingester-${id}"
description = "${d.name} data ingester for tpt-gov-nz (pulls from legacy ${d.name} systems, writes to dept DB)"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "gov-ingester-${id}"
path = "src/main.rs"

[dependencies]
gov-federation-core = { path = "../../crates/gov-federation-core" }
tokio = { workspace = true }
axum = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
sqlx = { workspace = true, features = ["macros", "migrate"] }
uuid = { workspace = true }
chrono = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
dotenvy = { workspace = true }
async-trait = "0.1"

[dev-dependencies]
tokio = { workspace = true }
uuid = { workspace = true }
`);

  // main.rs
  W(`${svc}/src/main.rs`, `use std::time::Duration;

use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use gov_ingester_${id}::{
    config::{Config, TransportKind},
    ingest,
    transport::{legacy::LegacyTransport, mock::MockTransport, IngesterTransport},
    IngestError,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("gov_ingester_${id}=info,sqlx=warn")),
        )
        .json()
        .init();

    let config = Config::from_env().map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let pool = PgPool::connect(&config.database_url).await?;

    sqlx::migrate!("../gov-dept-${id}/migrations")
        .run(&pool)
        .await?;
    info!("Migrations applied");

    let transport: Box<dyn IngesterTransport> = match config.transport {
        TransportKind::Mock => Box::new(MockTransport::new(&config.mock_fixture)),
        TransportKind::Legacy => Box::new(LegacyTransport::new(config.legacy.clone())),
    };

    if config.run_once {
        let summary = ingest::run_once_audited(&pool, transport.as_ref())
            .await
            .map_err(|e: IngestError| anyhow::anyhow!(e.to_string()))?;
        info!(
            source = %summary.source,
            citizens = summary.citizens_processed,
            inserted = summary.rows_inserted,
            updated = summary.rows_updated,
            "ingestion pass complete"
        );
        return Ok(());
    }

    let mut interval = tokio::time::interval(Duration::from_secs(config.interval_secs));
    info!(
        interval_secs = config.interval_secs,
        transport = ?config.transport,
        "ingester scheduler started"
    );
    loop {
        interval.tick().await;
        match ingest::run_once_audited(&pool, transport.as_ref()).await {
            Ok(summary) => info!(
                source = %summary.source,
                citizens = summary.citizens_processed,
                inserted = summary.rows_inserted,
                updated = summary.rows_updated,
                "ingestion pass complete"
            ),
            Err(e) => error!(error = %e, "ingestion pass failed"),
        }
    }
}
`);

  // lib.rs
  W(`${svc}/src/lib.rs`, `pub mod config;
pub mod db;
pub mod error;
pub mod ingest;
pub mod models;
pub mod raw;
pub mod transform;
pub mod transport;

pub use error::IngestError;

#[cfg(test)]
mod tests;
`);

  // error.rs
  W(`${svc}/src/error.rs`, `#[derive(Debug, thiserror::Error)]
pub enum IngestError {
    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Transform error: {0}")]
    Transform(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Transport not implemented: {0}")]
    NotImplemented(String),

    #[error("Config error: {0}")]
    Config(String),
}
`);

  // config.rs
  W(`${svc}/src/config.rs`, `use std::path::PathBuf;

use crate::{
    error::IngestError,
    transport::legacy::LegacyConfig,
};

#[derive(Debug, Clone)]
pub enum TransportKind {
    Mock,
    Legacy,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub transport: TransportKind,
    pub mock_fixture: PathBuf,
    pub legacy: LegacyConfig,
    pub interval_secs: u64,
    pub run_once: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, IngestError> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| IngestError::Config("DATABASE_URL must be set".into()))?;

        let transport = match std::env::var("${U}_TRANSPORT")
            .unwrap_or_else(|_| "mock".into())
            .to_lowercase()
            .as_str()
        {
            "legacy" => TransportKind::Legacy,
            _ => TransportKind::Mock,
        };

        let mock_fixture = PathBuf::from(
            std::env::var("${U}_MOCK_FIXTURE")
                .unwrap_or_else(|_| "fixtures/${id}_batch.json".into()),
        );

        let legacy = LegacyConfig {
            host: std::env::var("${U}_LEGACY_HOST")
                .unwrap_or_else(|_| "sftp.${id}.govt.nz".into()),
            port: std::env::var("${U}_LEGACY_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(22),
            username: std::env::var("${U}_LEGACY_USER").unwrap_or_default(),
            private_key_path: PathBuf::from(
                std::env::var("${U}_LEGACY_KEY").unwrap_or_default(),
            ),
            remote_path: std::env::var("${U}_LEGACY_REMOTE")
                .unwrap_or_else(|_| "/inbound/${id}_batch.json".into()),
        };

        let interval_secs = std::env::var("${U}_INGEST_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);

        let run_once = std::env::var("${U}_RUN_ONCE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Ok(Self {
            database_url,
            transport,
            mock_fixture,
            legacy,
            interval_secs,
            run_once,
        })
    }
}
`);

  // raw.rs
  const rawEntities = d.entities.map((e) => {
    const inner = e.fields.map((f) => `    pub ${f.n}: ${T[f.t].rust == "chrono::NaiveDate" ? "String" : T[f.t].rust},`).join("\n");
    const rawStruct = `#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Raw${pascal(e.key)} {
${inner}
}`;
    const citizenField = e.cardinality === "many"
      ? `    #[serde(default)]
    pub ${e.key}: Vec<Raw${pascal(e.key)}>,`
      : `    #[serde(default)]
    pub ${e.key}: Option<Raw${pascal(e.key)}>,`;
    return { rawStruct, citizenField };
  });

  W(`${svc}/src/raw.rs`, `//! Raw ${d.name} legacy data format.
//!
//! Mirrors the shape of a batch extract from the ${d.name} legacy systems.
//! Distinct from the department \`gov-dept-${id}\` DB schema; the [\`crate::transform\`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Raw${P}Batch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<Raw${P}Citizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Raw${P}Citizen {
    pub ${local}: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
${rawEntities.map((r) => r.citizenField).join("\n")}
}

${rawEntities.map((r) => r.rawStruct).join("\n\n")}
`);

  // models.rs
  const modelEntities = d.entities.map((e) => {
    const inner = e.fields.map((f) => `    pub ${f.n}: ${T[f.t].rust},`).join("\n");
    return `#[derive(Debug, Clone)]
pub struct ${pascal(e.key)}Entity {
${inner}
}`;
  }).join("\n\n");

  const transformedFields = d.entities.map((e) =>
    `    pub ${e.key}: ${e.cardinality === "many" ? `Vec<${pascal(e.key)}Entity>` : `Option<${pascal(e.key)}Entity>`},`
  ).join("\n");

  W(`${svc}/src/models.rs`, `use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestionSummary {
    pub source: String,
    pub batch_id: Option<String>,
    pub citizens_processed: u32,
    pub rows_inserted: u32,
    pub rows_updated: u32,
    pub status: IngestionStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IngestionStatus {
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct CitizenEntity {
    pub did: String,
    pub ${local}: String,
}

${modelEntities}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
${transformedFields}
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
`);

  // transform.rs
  const transformEntities = d.entities.map((e) => {
    if (e.cardinality === "many") {
      const assigns = e.fields.map((f) => {
        if (f.t === "date") return `                ${f.n}: parse_date(&c.${f.n}, "${f.n}")?,`;
        return `                ${f.n}: c.${f.n}.clone(),`;
      }).join("\n");
      return `    let ${e.key} = raw
        .${e.key}
        .iter()
        .map(|c| {
            Ok(${pascal(e.key)}Entity {
${assigns}
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;`;
    }
    const assigns = e.fields.map((f) => {
      if (f.t === "date") return `                ${f.n}: parse_date(&c.${f.n}, "${f.n}")?,`;
      return `                ${f.n}: c.${f.n}.clone(),`;
    }).join("\n");
    return `    let ${e.key} = match &raw.${e.key} {
        Some(c) => Some(${pascal(e.key)}Entity {
${assigns}
        }),
        None => None,
    };`;
  }).join("\n\n");

  const transformTests = d.entities.map((e) => {
    const fieldAsserts = e.fields.map((f) => {
      if (f.t === "string") return `         assert_eq!(t.${e.key === "many" ? e.key + "[0]" : e.key + ".as_ref().unwrap()"}.${f.n}, "${e.sample[f.j]}");`;
      if (f.t === "date") return `         assert_eq!(t.${e.key === "many" ? e.key + "[0]" : e.key + ".as_ref().unwrap()"}.${f.n}.to_string(), "${e.sample[f.j]}");`;
      return `         assert_eq!(t.${e.key === "many" ? e.key + "[0]" : e.key + ".as_ref().unwrap()"}.${f.n}, ${lit(e.sample[f.j])});`;
    }).join("\n");
    if (e.cardinality === "many") {
      return `    let t = transform_citizen(&batch.citizens[0]).unwrap();
${fieldAsserts}`;
    }
    return `    let t = transform_citizen(&batch.citizens[0]).unwrap();
${fieldAsserts}`;
  }).join("\n\n");

  W(`${svc}/src/transform.rs`, `//! Transform layer — maps the raw ${d.name} legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{${d.entities.map((e) => `${pascal(e.key)}Entity,`).join("")}CitizenEntity, TransformedCitizen},
    raw::Raw${P}Citizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &Raw${P}Citizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with ${local} {} has no DID",
            raw.${local}
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        ${local}: raw.${local}.clone(),
    };

${transformEntities}

    Ok(TransformedCitizen {
        citizen,
${d.entities.map((e) => `        ${e.key},`).join("\n")}
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Raw${P}Citizen {
        serde_json::from_value(serde_json::json!(${rawFixtureJson(d)}))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.${local}, "${d.localId.example}");
${transformTests}
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
`);

  // db.rs (ingester)
  const upserts = d.entities.map((e) => {
    const tbl = `${id}_${e.key}`;
    const biz = e.cardinality === "many" ? `citizen_id, ${e.fields[0].n}` : `citizen_id`;
    const setClause = e.fields.map((f) => `${f.n} = EXCLUDED.${f.n}`).join(", ");
    const params = e.fields.map((_, i) => `$${i + 2}`).join(", ");
    return `pub async fn upsert_${e.key}(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &${pascal(e.key)}Entity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO ${tbl} (citizen_id, ${fieldList(e)})
           VALUES ($1, ${params})
           ON CONFLICT (${biz}) DO UPDATE SET
              ${setClause}
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        ${e.fields.map((f) => `e.${f.n},`).join("\n        ")}
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}`;
  }).join("\n\n");

  W(`${svc}/src/db.rs`, `use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{${d.entities.map((e) => `${pascal(e.key)}Entity,`).join("")}IngestionStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub ${local}: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    ${local}: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, ${local})
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET ${local} = EXCLUDED.${local}
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        ${local},
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

${upserts}

pub async fn record_ingestion_run(
    pool: &PgPool,
    run_id: Uuid,
    source: &str,
    batch_id: Option<&str>,
    started_at: DateTime<Utc>,
    finished_at: DateTime<Utc>,
    citizens_processed: i32,
    rows_inserted: i32,
    rows_updated: i32,
    status: IngestionStatus,
    error_message: Option<&str>,
) -> sqlx::Result<()> {
    let status_str = match status {
        IngestionStatus::Running => "running",
        IngestionStatus::Success => "success",
        IngestionStatus::Failed => "failed",
    };
    sqlx::query!(
        r#"INSERT INTO ingestion_runs
              (id, source, batch_id, run_started_at, run_finished_at, citizens_processed,
               rows_inserted, rows_updated, status, error_message)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        run_id,
        source,
        batch_id,
        started_at,
        finished_at,
        citizens_processed,
        rows_inserted,
        rows_updated,
        status_str,
        error_message,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn latest_run(pool: &PgPool) -> sqlx::Result<Option<(String, i32, i32, String)>> {
    let row = sqlx::query!(
        r#"SELECT source, citizens_processed, rows_inserted, status
           FROM ingestion_runs ORDER BY run_started_at DESC LIMIT 1"#
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| (r.source, r.citizens_processed, r.rows_inserted, r.status)))
}
`);

  // ingest.rs
  const ingestEntities = d.entities.map((e) => {
    if (e.cardinality === "many") {
      return `        for ${e.key} in &t.${e.key} {
            count(
                db::upsert_${e.key}(pool, citizen_id, ${e.key}).await?,
                &mut inserted,
                &mut updated,
            );
        }`;
    }
    return `        if let Some(${e.key}) = &t.${e.key} {
            count(
                db::upsert_${e.key}(pool, citizen_id, ${e.key}).await?,
                &mut inserted,
                &mut updated,
            );
        }`;
  }).join("\n\n");

  W(`${svc}/src/ingest.rs`, `//! Ingestion orchestration — transport-agnostic.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db,
    error::IngestError,
    models::{new_run_id, IngestionStatus, IngestionSummary},
    transform::transform_citizen,
    transport::IngesterTransport,
};

pub async fn run_once(
    pool: &PgPool,
    transport: &dyn IngesterTransport,
) -> Result<IngestionSummary, IngestError> {
    let run_id: Uuid = new_run_id();
    let started = Utc::now();

    let batch = transport.pull().await?;
    let source = batch.source.clone();
    let batch_id = Some(batch.batch_id.clone());

    let mut inserted = 0u32;
    let mut updated = 0u32;

    for raw in &batch.citizens {
        let t = transform_citizen(raw)?;

        let (citizen_id, was_inserted) =
            db::upsert_citizen(pool, &t.citizen.did, &t.citizen.${local}).await?;
        if was_inserted {
            inserted += 1;
        } else {
            updated += 1;
        }

${ingestEntities}
    }

    let citizens_processed = batch.citizens.len() as u32;
    let finished = Utc::now();

    db::record_ingestion_run(
        pool,
        run_id,
        &source,
        batch_id.as_deref(),
        started,
        finished,
        citizens_processed as i32,
        inserted as i32,
        updated as i32,
        IngestionStatus::Success,
        None,
    )
    .await?;

    Ok(IngestionSummary {
        source,
        batch_id,
        citizens_processed,
        rows_inserted: inserted,
        rows_updated: updated,
        status: IngestionStatus::Success,
        error_message: None,
    })
}

#[inline]
fn count(was_inserted: bool, inserted: &mut u32, updated: &mut u32) {
    if was_inserted {
        *inserted += 1;
    } else {
        *updated += 1;
    }
}

pub async fn run_once_audited(
    pool: &PgPool,
    transport: &dyn IngesterTransport,
) -> Result<IngestionSummary, IngestError> {
    match run_once(pool, transport).await {
        Ok(summary) => Ok(summary),
        Err(e) => {
            let run_id = new_run_id();
            let now = Utc::now();
            let msg = e.to_string();
            db::record_ingestion_run(
                pool,
                run_id,
                "unknown",
                None,
                now,
                now,
                0,
                0,
                0,
                IngestionStatus::Failed,
                Some(&msg),
            )
            .await
            .ok();
            Err(e)
        }
    }
}
`);

  // transport.rs
  W(`${svc}/src/transport.rs`, `pub mod legacy;
pub mod mock;

use async_trait::async_trait;

use crate::{error::IngestError, raw::Raw${P}Batch};

/// Pluggable ingestion source for the ${d.name} ingester.
#[async_trait]
pub trait IngesterTransport: Send + Sync {
    async fn pull(&self) -> Result<Raw${P}Batch, IngestError>;
}
`);

  // transport/mock.rs
  W(`${svc}/src/transport/mock.rs`, `use std::path::PathBuf;

use async_trait::async_trait;

use crate::{
    error::IngestError,
    raw::Raw${P}Batch,
    transport::IngesterTransport,
};

/// Reads a raw ${d.name} batch from a JSON fixture file on disk (dev/demo/CI).
pub struct MockTransport {
    fixture_path: PathBuf,
}

impl MockTransport {
    pub fn new(fixture_path: impl Into<PathBuf>) -> Self {
        Self {
            fixture_path: fixture_path.into(),
        }
    }
}

#[async_trait]
impl IngesterTransport for MockTransport {
    async fn pull(&self) -> Result<Raw${P}Batch, IngestError> {
        let bytes = tokio::fs::read(&self.fixture_path).await.map_err(|e| {
            IngestError::Transport(format!(
                "failed to read fixture {}: {e}",
                self.fixture_path.display()
            ))
        })?;
        let batch = serde_json::from_slice::<Raw${P}Batch>(&bytes)
            .map_err(|e| IngestError::Transform(format!("invalid fixture JSON: {e}")))?;
        Ok(batch)
    }
}
`);

  // transport/legacy.rs
  W(`${svc}/src/transport/legacy.rs`, `use std::path::PathBuf;

use async_trait::async_trait;

use crate::{
    error::IngestError,
    raw::Raw${P}Batch,
    transport::IngesterTransport,
};

/// Connection details for the real ${d.name} legacy system integration.
#[derive(Debug, Clone)]
pub struct LegacyConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub private_key_path: PathBuf,
    pub remote_path: String,
}

pub struct LegacyTransport {
    config: LegacyConfig,
}

impl LegacyTransport {
    pub fn new(config: LegacyConfig) -> Self {
        Self { config }
    }

    #[allow(dead_code)]
    fn connection_summary(&self) -> String {
        format!(
            "${id}-legacy://{}@{}:{}{}",
            self.config.username, self.config.host, self.config.port, self.config.remote_path
        )
    }
}

#[async_trait]
impl IngesterTransport for LegacyTransport {
    async fn pull(&self) -> Result<Raw${P}Batch, IngestError> {
        Err(IngestError::NotImplemented(format!(
            "${d.name} legacy transport not yet wired up (target: {}). \
             Implement the legacy system download in production; the transform + \
             upsert pipeline is transport-agnostic and ready to consume the result.",
            self.connection_summary()
        )))
    }
}
`);

  // tests.rs (ingester)
  const fixtureTestAsserts = d.entities.map((e) => {
    if (e.cardinality === "many") {
      return `    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.${e.key}.len(), 1);
    assert_eq!(t.${e.key}[0].${e.fields[0].n}, ${lit(e.sample[e.fields[0].j])});`;
    }
    return `    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert!(t.${e.key}.is_some());
    assert_eq!(t.${e.key}.as_ref().unwrap().${e.fields[0].n}, ${lit(e.sample[e.fields[0].j])});`;
  }).join("\n\n");

  W(`${svc}/src/tests.rs`, `//! Unit tests for the ${d.name} ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::Raw${P}Batch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/${id}_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: Raw${P}Batch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.${local}, "${d.localId.example}");
${fixtureTestAsserts}
}

#[test]
fn missing_did_is_rejected() {
    let raw: Raw${P}Batch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "${local}": "${d.localId.example}", "did": "", ${d.entities.map((e) => e.cardinality === "many" ? `"${e.key}": []` : `"${e.key}": null`).join(", ")} }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
`);

  // fixture
  W(`${svc}/fixtures/${id}_batch.json`, JSON.stringify(rawFixture(d), null, 2) + "\n");

  // Dockerfile (ingester)
  W(`${svc}/Dockerfile`, `FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-${id}/ services/gov-dept-${id}/
COPY services/gov-ingester-${id}/ services/gov-ingester-${id}/

# Build the ${d.name} ingester
RUN cargo build --release -p gov-ingester-${id}

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-${id} /usr/local/bin/gov-ingester-${id}
COPY --from=builder /app/services/gov-ingester-${id}/fixtures /fixtures

CMD ["gov-ingester-${id}"]
`);
}

function rawFixtureJson(d) {
  const citizen1 = { [`${d.localId.field}`]: d.localId.example, did: "did:gov:nz:test-citizen-001", name: "Alex Tane" };
  for (const e of d.entities) {
    if (e.cardinality === "many") citizen1[e.key] = [e.sample];
    else citizen1[e.key] = e.sample;
  }
  const citizen2 = { [`${d.localId.field}`]: d.localId.example.replace(/001$/, "002"), did: "did:gov:nz:test-citizen-002", name: "Bree Kare" };
  for (const e of d.entities) {
    if (e.cardinality === "many") citizen2[e.key] = [];
    else citizen2[e.key] = null;
  }
  return { batchId: `BATCH-${d.id.toUpperCase()}-001`, generatedAt: "2026-06-01T00:00:00Z", source: "mock", citizens: [citizen1, citizen2] };
}

function rawFixture(d) {
  return rawFixtureJson(d);
}

function genSchema(d) {
  const P = pascal(d.id);
  const id = d.id;
  const schemas = d.entities.map((e) => {
    const fields = e.fields.map((f) => `  ${f.j}: ${T[f.t].zod},`).join("\n");
    return `export const ${P}${pascal(e.key)}Schema = z.object({\n${fields}\n});`;
  }).join("\n\n");

  const bundleFields = d.entities.map((e) =>
    e.cardinality === "many"
      ? `  ${e.key}: z.array(${P}${pascal(e.key)}Schema).optional(),`
      : `  ${e.key}: ${P}${pascal(e.key)}Schema.optional(),`
  ).join("\n");

  const actionMembers = d.actions.map((a) => {
    const params = (a.params || []).map((p) => `    ${p.name}: z.string().min(1),`).join("\n");
    return `  z.object({\n    type: z.literal("${a.type}"),\n${params}\n  }),`;
  }).join("\n");

  W(`packages/@tpt/gov-schema/src/departments/${id}.ts`, `import { z } from "zod";

${schemas}

export const ${P}DataBundleSchema = z.object({
  ${id}Id: z.string(),
${bundleFields}
});

export type ${P}DataBundle = z.infer<typeof ${P}DataBundleSchema>;

export const ${P}ActionSchema = z.discriminatedUnion("type", [
${actionMembers}
]);

export type ${P}Action = z.infer<typeof ${P}ActionSchema>;
`);
}

function genPolicy(d) {
  const id = d.id;
  const firstScope = d.entities[0].scope;
  W(`policies/${id}.rego`, `package ${id}.consent

# Consent policy for the ${d.name} department node.
#
# Mirrors \`src/consent.rs\` in \`gov-dept-${id}\`. Every cross-department DATA_REQUEST
# arriving at /citizen/data must carry signed DataGrantCredentials; a request is
# allowed only if every requested scope is covered by a non-expired grant whose
# requesting/providing departments and citizen DID match the request.
#
# Citizen (self) access and staff (case-worker) access are authorised directly by
# the ${id} node because the citizen has implicit consent over their own record.

import rego.v1

default allow := false

allow if {
    every scope in input.requested_scopes {
        scope_covered(scope)
    }
}

scope_covered(scope) if {
    some grant in input.consent_grants
    grant.requestingDeptId == input.requesting_dept_id
    grant.providingDeptId == input.providing_dept_id
    grant.citizenDid == input.citizen_did
    grant.expiresAt > time.now_ns() / 1000000000
    scope in grant.scopes
}

denied_scopes contains scope if {
    scope := input.requested_scopes[_]
    not scope_covered(scope)
}
`);
}

function genPortal(d) {
  const id = d.id;
  const P = pascal(id);
  const U = upper(id);
  const dir = `apps/portal-citizen/app/dept/${id}`;

  // actions.ts
  W(`${dir}/actions.ts`, `"use server";

import type { ${P}DataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produce${P}AiContext } from "./${id}-ai";

const ${U}_SERVICE_URL = process.env.${U}_SERVICE_URL ?? "http://localhost:${d.port}";

export async function fetch${P}Data(scopes: string[]): Promise<${P}DataBundle | null> {
  return (await fetchDeptData("${id}", scopes)) as ${P}DataBundle | null;
}

export async function submit${P}Action(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(${U}_SERVICE_URL + "/citizen/action", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, type, parameters, performed_by: "citizen" }),
    });

    if (!res.ok) {
      const err = (await res.json()) as { error?: string };
      return { success: false, message: err.error ?? "Action failed" };
    }

    return res.json() as Promise<{ success: boolean; message?: string }>;
  } catch {
    return { success: false, message: "Service unavailable" };
  }
}

// ── AI ─────────────────────────────────────────────────────────────

export async function ask${P}(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetch${P}Data([${d.entities.map((e) => `"${e.scope}"`).join(", ")}]);
  if (!data) {
    return { answer: "Unable to load your ${d.name} information.", enabled: false };
  }
  const context = produce${P}AiContext(data);
  return askWithContext(question, context);
}
`);

  // <id>-ai.ts
  const chunks = d.entities.map((e) => {
    if (e.cardinality === "many") {
      const desc = e.fields.map((f) => `x.${f.j}`).join(' + ", " + ');
      return `  if (data.${e.key} && data.${e.key}.length > 0) {
    for (const x of data.${e.key}) {
      chunks.push({
        deptId: "${id}",
        content: "${e.label}: " + ${desc},
        metadata: { area: "${e.key}" },
      });
    }
  }`;
    }
    const desc = e.fields.map((f) => `data.${e.key}.${f.j}`).join(' + ", " + ');
    return `  if (data.${e.key}) {
    chunks.push({
      deptId: "${id}",
      content: "${e.label}: " + ${desc},
      metadata: { area: "${e.key}" },
    });
  }`;
  }).join("\n\n");

  W(`${dir}/${id}-ai.ts`, `import type { ${P}DataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produce${P}AiContext(data: ${P}DataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "${id}",
    content: "${d.name} local ID: " + data.${id}Id + ".",
    metadata: { area: "${id}-id" },
  });

${chunks}

  return chunks;
}
`);

  // ai-prompt.tsx
  W(`${dir}/ai-prompt.tsx`, `"use client";

import { useState } from "react";
import { ask${P} } from "./actions";

export default function ${P}AiPrompt() {
  const [question, setQuestion] = useState("${d.aiPrompt}");
  const [answer, setAnswer] = useState<string | null>(null);
  const [enabled, setEnabled] = useState(true);
  const [asking, setAsking] = useState(false);

  async function handleAsk(e: React.FormEvent) {
    e.preventDefault();
    setAsking(true);
    setAnswer(null);
    const res = await ask${P}(question);
    setEnabled(res.enabled);
    setAnswer(res.answer);
    setAsking(false);
  }

  return (
    <section style={{ marginTop: "1.5rem" }}>
      <h2>Ask about your ${d.shortName} information</h2>
      <form onSubmit={handleAsk}>
        <label htmlFor="${id}-q" style={{ display: "block", marginBottom: "0.5rem" }}>
          Your question
          <input
            id="${id}-q"
            type="text"
            value={question}
            onChange={(e) => setQuestion(e.target.value)}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
          />
        </label>
        <button type="submit" disabled={asking}>
          {asking ? "Asking..." : "Ask AI"}
        </button>
      </form>

      {answer != null && (
        <p style={{ marginTop: "1rem", whiteSpace: "pre-wrap" }}>
          <strong>AI:</strong> {answer}
          {!enabled && <em> (AI is disabled — enable an AI level to get a live answer.)</em>}
        </p>
      )}
    </section>
  );
}
`);

  // page.tsx (overview)
  const sections = d.entities.map((e) => `      <section>
        <h2>${e.label}</h2>
        {data.${e.key} ? (
          <Link href={"/dept/${id}/${e.key}"}>View ${e.label.toLowerCase()} →</Link>
        ) : (
          <p>No ${e.label.toLowerCase()} on file.</p>
        )}
      </section>`).join("\n\n");

  W(`${dir}/page.tsx`, `import Link from "next/link";
import { fetch${P}Data } from "./actions";
import ${P}AiPrompt from "./ai-prompt";

export const metadata = { title: "${d.name} — My Gov NZ" };

export default async function ${P}OverviewPage() {
  const data = await fetch${P}Data([${d.entities.map((e) => `"${e.scope}"`).join(", ")}]);

  if (!data) {
    return (
      <main>
        <h1>${d.name}</h1>
        <p>Unable to load your ${d.shortName} information. Please grant access to continue.</p>
        <Link href={"/consent?dept=${id}"}>Grant ${d.shortName} access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>${d.name}</h1>
      <p>${d.shortName} ID: ••••{data.${id}Id.slice(-4)}</p>

${sections}

      <${P}AiPrompt />
    </main>
  );
}
`);

  // loading.tsx
  W(`${dir}/loading.tsx`, `export default function Loading() {
  return (
    <main>
      <h1>${d.name}</h1>
      <p>Loading your ${d.shortName} information…</p>
    </main>
  );
}
`);

  // per-entity subpages (citizen)
  for (const e of d.entities) {
    const edir = `${dir}/${e.key}`;
    if (e.cardinality === "many") {
      const head = e.fields.map((f) => `                <th>${f.j}</th>`).join("\n");
      const row = e.fields.map((f) => `                  <td>{row.${f.j}}</td>`).join("\n");
      W(`${edir}/page.tsx`, `import Link from "next/link";
import { fetch${P}Data } from "../actions";

export const metadata = { title: "${e.label} — ${d.name} — My Gov NZ" };

export default async function ${P}${pascal(e.key)}Page() {
  const data = await fetch${P}Data(["${e.scope}"]);
  if (!data) {
    return (
      <main>
        <h1>${e.label}</h1>
        <p>Unable to load your ${d.shortName} information.</p>
        <Link href={"/consent?dept=${id}"}>Grant ${d.shortName} access</Link>
      </main>
    );
  }

  const rows = data.${e.key} ?? [];

  return (
    <main>
      <Link href={"/dept/${id}"}>← Back to ${d.shortName}</Link>
      <h1>${e.label}</h1>
      {rows.length === 0 ? (
        <p>No ${e.label.toLowerCase()} on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
${head}
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
${row}
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
`);
    } else {
      const fields = e.fields.map((f) => `        <p><strong>${f.j}:</strong> {item.${f.j}}</p>`).join("\n");
      W(`${edir}/page.tsx`, `import Link from "next/link";
import { fetch${P}Data } from "../actions";

export const metadata = { title: "${e.label} — ${d.name} — My Gov NZ" };

export default async function ${P}${pascal(e.key)}Page() {
  const data = await fetch${P}Data(["${e.scope}"]);
  if (!data) {
    return (
      <main>
        <h1>${e.label}</h1>
        <p>Unable to load your ${d.shortName} information.</p>
        <Link href={"/consent?dept=${id}"}>Grant ${d.shortName} access</Link>
      </main>
    );
  }

  const item = data.${e.key};

  return (
    <main>
      <Link href={"/dept/${id}"}>← Back to ${d.shortName}</Link>
      <h1>${e.label}</h1>
      {item ? (
        <div>
${fields}
        </div>
      ) : (
        <p>No ${e.label.toLowerCase()} on file.</p>
      )}
    </main>
  );
}
`);
    }
  }
}

function genStaff(d) {
  const id = d.id;
  const P = pascal(id);
  const U = upper(id);
  const dir = `apps/portal-staff/app/dept/${id}`;

  // actions.ts
  W(`${dir}/actions.ts`, `"use server";

import type { ${P}DataBundle } from "@tpt/gov-schema";

const ${U}_SERVICE_URL = process.env.${U}_SERVICE_URL ?? "http://localhost:${d.port}";

/**
 * Read-only fetch of a citizen's ${d.name} data, performed by a case worker.
 */
export async function fetch${P}DataForCitizen(
  did: string,
  scopes: string[],
): Promise<${P}DataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(${U}_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<${P}DataBundle>;
  } catch {
    return null;
  }
}
`);

  // page.tsx (overview)
  const sections = d.entities.map((e) => `      <section>
        <h2>${e.label}</h2>
        {data && data.${e.key} ? (
          <Link href={\`/dept/${id}/${e.key}?did=\${encodeURIComponent(did)}\`}>View ${e.label.toLowerCase()}</Link>
        ) : (
          <p>No ${e.label.toLowerCase()} on file.</p>
        )}
      </section>`).join("\n\n");

  W(`${dir}/page.tsx`, `import Link from "next/link";
import { fetch${P}DataForCitizen } from "./actions";

export const metadata = { title: "${d.name} — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function ${P}StaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>${d.name} — Case File</h1>
        <p>No citizen selected. Enter a DID to view their ${d.shortName} records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetch${P}DataForCitizen(did, [${d.entities.map((e) => `"${e.scope}"`).join(", ")}]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>${d.name} — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load ${d.shortName} information for this citizen.</p>}

      {data && (
        <>
          <p>${d.shortName} ID: ••••{data.${id}Id.slice(-4)}</p>

${sections}
        </>
      )}
    </main>
  );
}
`);

  // per-entity subpages (staff)
  for (const e of d.entities) {
    const edir = `${dir}/${e.key}`;
    if (e.cardinality === "many") {
      const head = e.fields.map((f) => `                <th>${f.j}</th>`).join("\n");
      const row = e.fields.map((f) => `                  <td>{row.${f.j}}</td>`).join("\n");
      W(`${edir}/page.tsx`, `import { fetch${P}DataForCitizen } from "../actions";

export const metadata = { title: "${e.label} — ${d.name} — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function ${P}${pascal(e.key)}StaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetch${P}DataForCitizen(did, ["${e.scope}"]);
  const rows = data?.${e.key} ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={\`/dept/${id}?did=\${encodeURIComponent(did)}\`}>← Back to ${d.shortName} case file</Link>
      <h1>${e.label}</h1>
      {rows.length === 0 ? (
        <p>No ${e.label.toLowerCase()} on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
${head}
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
${row}
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
`);
    } else {
      const fields = e.fields.map((f) => `        <p><strong>${f.j}:</strong> {item.${f.j}}</p>`).join("\n");
      W(`${edir}/page.tsx`, `import { fetch${P}DataForCitizen } from "../actions";

export const metadata = { title: "${e.label} — ${d.name} — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function ${P}${pascal(e.key)}StaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetch${P}DataForCitizen(did, ["${e.scope}"]);
  const item = data?.${e.key};

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={\`/dept/${id}?did=\${encodeURIComponent(did)}\`}>← Back to ${d.shortName} case file</Link>
      <h1>${e.label}</h1>
      {item ? (
        <div>
${fields}
        </div>
      ) : (
        <p>No ${e.label.toLowerCase()} on file.</p>
      )}
    </main>
  );
}
`);
    }
  }
}

// ── shared-file patching ──────────────────────────────────────────────────────────
function patchFile(rel, anchor, insertion, guard, mode = "after") {
  const content = R(rel);
  if (guard && content.includes(guard)) return;
  const idx = content.indexOf(anchor);
  if (idx < 0) {
    console.warn(`WARN: anchor not found in ${rel}: ${JSON.stringify(anchor.slice(0, 50))}`);
    return;
  }
  let updated;
  if (mode === "before") {
    updated = content.slice(0, idx) + insertion + content.slice(idx);
  } else {
    updated = content.slice(0, idx) + anchor + insertion + content.slice(idx + anchor.length);
  }
  writeFileSync(join(ROOT, rel), updated);
}

function bundleJson(d) {
  const obj = { [`${d.id}Id`]: d.localId.example };
  for (const e of d.entities) {
    obj[e.key] = e.cardinality === "many" ? [e.sample] : e.sample;
  }
  return JSON.stringify(obj, null, 2);
}

function patchAll(depts) {
  for (const d of depts) {
    const id = d.id;
    const P = pascal(id);
    const U = upper(id);
    const localEx = d.localId.example;

    // schema index
    patchFile(
      "packages/@tpt/gov-schema/src/departments/index.ts",
      `export * from "./adapter.js";`,
      `\nexport * from "./${id}.js";`,
      `./${id}.js`
    );

    // data-access union
    patchFile(
      "apps/portal-citizen/app/lib/data-access.ts",
      `  | import("@tpt/gov-schema").TPKDataBundle;`,
      `\n  | import("@tpt/gov-schema").${P}DataBundle;`,
      `${P}DataBundle;`
    );

    // citizen config: services + DeptId + DEPARTMENTS
    patchFile(
      "apps/portal-citizen/app/lib/config.ts",
      `    tpk: process.env.TPK_SERVICE_URL ?? "http://localhost:8108",`,
      `\n    ${id}: process.env.${U}_SERVICE_URL ?? "http://localhost:${d.port}",`,
      `${id}: process.env.`
    );
    patchFile(
      "apps/portal-citizen/app/lib/config.ts",
      `| "doc" | "tpk"`,
      ` | "${id}"`,
      `"${id}";`
    );
    patchFile(
      "apps/portal-citizen/app/lib/config.ts",
      `export const DEPARTMENTS: DeptMeta[] = [`,
      `\n  {
    id: "${id}",
    name: "${d.name}",
    shortName: "${d.shortName}",
    description: "${d.description}",
    scopes: [${d.entities.map((e) => `"${e.scope}"`).join(", ")}],
    href: "/dept/${id}",
  },`,
      `id: "${id}",`
    );

    // staff config
    patchFile(
      "apps/portal-staff/app/lib/config.ts",
      `    tpk: process.env.TPK_SERVICE_URL ?? "http://localhost:8108",`,
      `\n    ${id}: process.env.${U}_SERVICE_URL ?? "http://localhost:${d.port}",`,
      `${id}: process.env.`
    );
    patchFile(
      "apps/portal-staff/app/lib/config.ts",
      `| "doc" | "tpk"`,
      ` | "${id}"`,
      `"${id}";`
    );
    patchFile(
      "apps/portal-staff/app/lib/config.ts",
      `export const STAFF_DEPARTMENTS: StaffDeptMeta[] = [`,
      `\n  {
    id: "${id}",
    name: "${d.name}",
    shortName: "${d.shortName}",
    description: "${d.description}",
    scopes: [${d.entities.map((e) => `"${e.scope}"`).join(", ")}],
    href: "/dept/${id}",
  },`,
      `id: "${id}",`
    );

    // mock-data
    const consts = `const ${id}Standard: ${P}DataBundle = ${bundleJson(d)};
const ${id}Beneficiary: ${P}DataBundle = ${id}Standard;
const ${id}NewParent: ${P}DataBundle = ${id}Standard;

`;
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `  TPKDataBundle,`,
      `\n  ${P}DataBundle,`,
      `${P}DataBundle,`
    );
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `  tpk: TPKDataBundle;`,
      `\n  ${id}: ${P}DataBundle;`,
      `  ${id}: ${P}DataBundle;`
    );
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `export function getDemoData`,
      `${consts}`,
      `${id}Standard:`,
      "before"
    );
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `        tpk: tpkBeneficiary,`,
      `\n        ${id}: ${id}Beneficiary,`,
      `        ${id}Beneficiary,`,
      "before"
    );
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `        tpk: tpkNewParent,`,
      `\n        ${id}: ${id}NewParent,`,
      `        ${id}NewParent,`,
      "before"
    );
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `        tpk: tpkStandard,`,
      `\n        ${id}: ${id}Standard,`,
      `        ${id}Standard,`,
      "before"
    );
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `        tpk: tpkNewParent,`,
      `        ${id}: ${id}NewParent,\n        tpk: tpkNewParent,`,
      `${id}NewParent,`
    );
    patchFile(
      "apps/portal-citizen/app/lib/mock-data.ts",
      `        tpk: tpkStandard,`,
      `        ${id}: ${id}Standard,\n        tpk: tpkStandard,`,
      `${id}Standard,`
    );

    // .env.example
    patchFile(
      ".env.example",
      `TPK_SERVICE_URL=http://localhost:8108`,
      `\n${U}_SERVICE_URL=http://localhost:${d.port}`,
      `${U}_SERVICE_URL=`
    );
    patchFile(
      ".env.example",
      `# STATSNZ_LEGACY_REMOTE=/inbound/statsnz_batch.json`,
      `\n# ${U}_TRANSPORT=mock              # mock | legacy
# ${U}_MOCK_FIXTURE=./services/gov-ingester-${id}/fixtures/${id}_batch.json
# ${U}_RUN_ONCE=true
# ${U}_INGEST_INTERVAL_SECS=3600
# ${U}_LEGACY_HOST=sftp.${id}.govt.nz
# ${U}_LEGACY_KEY=/path/to/priv.key
# ${U}_LEGACY_REMOTE=/inbound/${id}_batch.json`,
      `${U}_TRANSPORT=mock`
    );

    // docker phase1.yml
    const deptBlock = `
  dept-${id}:
    image: postgres:17-alpine
    environment:
      POSTGRES_DB: ${id}
      POSTGRES_USER: ${id}
      POSTGRES_PASSWORD: ${id}_dev_password
    ports:
      - "${d.dbPort}:5432"
    volumes:
      - ${id}-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${id}"]
      interval: 5s
      timeout: 5s
      retries: 10

  gov-dept-${id}:
    build:
      context: ..
      dockerfile: services/gov-dept-${id}/Dockerfile
    environment:
      DATABASE_URL: postgresql://${id}:${id}_dev_password@dept-${id}:5432/${id}
      TPT__GOV__DEPT_ID: ${id}
      TPT__GOV__HTTP_LISTEN: "0.0.0.0:${d.port}"
      TPT__GOV__FEDERATION_LISTEN: "0.0.0.0:${d.fedPort}"
      RUST_LOG: "gov_dept_${id}=info,tower_http=debug"
    ports:
      - "${d.port}:${d.port}"
    depends_on:
      dept-${id}:
        condition: service_healthy

  gov-ingester-${id}:
    build:
      context: ..
      dockerfile: services/gov-ingester-${id}/Dockerfile
    environment:
      DATABASE_URL: postgresql://${id}:${id}_dev_password@dept-${id}:5432/${id}
      ${U}_TRANSPORT: mock
      ${U}_MOCK_FIXTURE: /fixtures/${id}_batch.json
      ${U}_RUN_ONCE: "true"
      RUST_LOG: "gov_ingester_${id}=info,sqlx=warn"
    depends_on:
      dept-${id}:
        condition: service_healthy

`;
    patchFile(
      "docker/phase1.yml",
      `  gov-gateway:`,
      deptBlock + `  gov-gateway:`,
      `gov-dept-${id}:`
    );
    patchFile(
      "docker/phase1.yml",
      `      TPT__GOV__DEPT_TPK_URL: "http://gov-dept-tpk:8108"`,
      `\n      TPT__GOV__DEPT_${U}_URL: "http://gov-dept-${id}:${d.port}"`,
      `TPT__GOV__DEPT_${U}_URL:`
    );
    patchFile(
      "docker/phase1.yml",
      `  tpk-data:`,
      `  ${id}-data:\n  tpk-data:`,
      `  ${id}-data:`
    );

    // Cargo.toml members
    patchFile(
      "Cargo.toml",
      `\n]`,
      `\n    "services/gov-dept-${id}",\n    "services/gov-ingester-${id}",`,
      `"services/gov-dept-${id}"`,
      "before"
    );
  }
}

function genDepartment(d) {
  genDeptService(d);
  genIngester(d);
  genSchema(d);
  genPolicy(d);
  genPortal(d);
  genStaff(d);
}

const only = process.argv[2];
const selected = only ? DEPTS.filter((d) => d.id === only) : DEPTS;
for (const d of selected) {
  genDepartment(d);
}
patchAll(selected);
console.log(`Generated ${selected.length} department(s): ${selected.map((d) => d.id).join(", ")}`);

export { genDeptService, genIngester, genSchema, genPolicy, genPortal, genStaff, genDepartment, patchAll, lit, sampleVal, rawFixtureJson, rawFixture };
