//! Audit logging for the generic department node.
//!
//! Every data access (resolve, fetch, action) is recorded in an append-only
//! `audit_log` table. The schema is created on startup via
//! [`ensure_schema`], so the template compiles and runs without a pre-seeded
//! migrations directory (unlike the per-department services, which use
//! compile-time `sqlx::migrate!`).
//!
//! Queries use the runtime `sqlx::query` API (not the `query!` macro) so the
//! node does not require a live database at compile time.

use sqlx::PgPool;

/// Outcome of an audited access decision.
#[derive(Debug, Clone, Copy)]
pub enum Decision {
    Allowed,
    Denied,
}

impl Decision {
    fn as_str(&self) -> &'static str {
        match self {
            Decision::Allowed => "allowed",
            Decision::Denied => "denied",
        }
    }
}

/// One audited event.
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub dept_id: String,
    pub citizen_did: String,
    /// The operation performed: "resolve", "fetch_data", "submit_action", …
    pub action: String,
    /// Department requesting the data, when this is a cross-department access.
    pub requesting_dept: Option<String>,
    /// Scopes that were requested (for `fetch_data`).
    pub scopes: Vec<String>,
    pub decision: Decision,
    /// Free-text detail, e.g. the denied scope list or the action type.
    pub detail: Option<String>,
}

/// Create the `audit_log` table if it does not already exist.
pub async fn ensure_schema(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS audit_log (
            id              BIGSERIAL PRIMARY KEY,
            occurred_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
            dept_id         TEXT NOT NULL,
            citizen_did     TEXT NOT NULL,
            action          TEXT NOT NULL,
            requesting_dept TEXT,
            scopes          TEXT NOT NULL DEFAULT '',
            decision        TEXT NOT NULL,
            detail          TEXT
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Append an audit event. Errors are logged but never propagated — an audit
/// write failing must not block the user's request.
pub async fn log_access(pool: &PgPool, event: &AuditEvent) {
    let scopes = event.scopes.join(",");
    let result = sqlx::query(
        "INSERT INTO audit_log (dept_id, citizen_did, action, requesting_dept, scopes, decision, detail)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(&event.dept_id)
    .bind(&event.citizen_did)
    .bind(&event.action)
    .bind(&event.requesting_dept)
    .bind(&scopes)
    .bind(event.decision.as_str())
    .bind(&event.detail)
    .execute(pool)
    .await;

    if let Err(e) = result {
        tracing::warn!(error = %e, "failed to write audit_log entry");
    }
}
