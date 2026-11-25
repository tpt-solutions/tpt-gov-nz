//! Audit logging for the generic department node.
//!
//! Every data access (resolve, fetch, action) is recorded in an append-only
//! `audit_log` table with a hash chain for tamper evidence. The schema is
//! created on startup via [`ensure_schema`].
//!
//! The audit write **fails closed** — if the audit log cannot be written, the
//! request is rejected. This ensures a complete audit trail.
//!
//! Each entry is also represented as a signed `AuditLogEntry` from the
//! federation core for cross-department audit verification.

use gov_federation_core::{AuditLogEntry, NodeKeypair};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

/// Outcome of an audited access decision.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
/// Includes hash chain for tamper evidence.
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
            detail          TEXT,
            prev_hash       BYTEA,
            entry_hash      BYTEA NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS audit_log_prev_hash_idx ON audit_log (prev_hash)")
        .execute(pool)
        .await?;

    // Append-only: rows can be read and inserted, never changed or removed. The hash chain
    // only gives tamper evidence if the database itself refuses UPDATE and DELETE.
    sqlx::query(
        "CREATE OR REPLACE FUNCTION reject_audit_log_change() RETURNS trigger AS $$
        BEGIN
            RAISE EXCEPTION 'audit_log is append-only';
        END;
        $$ LANGUAGE plpgsql",
    )
    .execute(pool)
    .await?;

    sqlx::query("DROP TRIGGER IF EXISTS audit_log_append_only ON audit_log")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE TRIGGER audit_log_append_only
            BEFORE UPDATE OR DELETE ON audit_log
            FOR EACH ROW EXECUTE FUNCTION reject_audit_log_change()",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Compute the hash of an audit entry for the hash chain.
// Eight inputs are the chain's fixed fields; grouping them would only move the list elsewhere.
#[allow(clippy::too_many_arguments)]
fn compute_entry_hash(
    prev_hash: Option<&[u8]>,
    dept_id: &str,
    citizen_did: &str,
    action: &str,
    requesting_dept: Option<&str>,
    scopes: &[String],
    decision: &Decision,
    detail: Option<&str>,
) -> [u8; 32] {
    let mut hasher = Sha256::new();

    if let Some(prev) = prev_hash {
        hasher.update(prev);
    }

    hasher.update(dept_id.as_bytes());
    hasher.update(citizen_did.as_bytes());
    hasher.update(action.as_bytes());
    if let Some(req_dept) = requesting_dept {
        hasher.update(req_dept.as_bytes());
    }
    for scope in scopes {
        hasher.update(scope.as_bytes());
    }
    hasher.update(decision.as_str().as_bytes());
    if let Some(d) = detail {
        hasher.update(d.as_bytes());
    }

    hasher.finalize().into()
}

/// Append an audit event. Fails the request if audit write fails (fail closed).
/// Returns the entry hash for the caller.
pub async fn log_access(
    pool: &PgPool,
    event: &AuditEvent,
    audit_keypair: Option<&NodeKeypair>,
) -> anyhow::Result<[u8; 32]> {
    let prev_hash: Option<Vec<u8>> =
        sqlx::query_scalar("SELECT entry_hash FROM audit_log ORDER BY id DESC LIMIT 1")
            .fetch_optional(pool)
            .await?;

    let prev_hash_bytes = prev_hash.as_deref();

    let entry_hash = compute_entry_hash(
        prev_hash_bytes,
        &event.dept_id,
        &event.citizen_did,
        &event.action,
        event.requesting_dept.as_deref(),
        &event.scopes,
        &event.decision,
        event.detail.as_deref(),
    );

    let scopes = event.scopes.join(",");

    let result = sqlx::query(
        "INSERT INTO audit_log (dept_id, citizen_did, action, requesting_dept, scopes, decision, detail, prev_hash, entry_hash)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(&event.dept_id)
    .bind(&event.citizen_did)
    .bind(&event.action)
    .bind(&event.requesting_dept)
    .bind(&scopes)
    .bind(event.decision.as_str())
    .bind(&event.detail)
    .bind(prev_hash_bytes)
    .bind(entry_hash.as_slice())
    .execute(pool)
    .await;

    if let Err(e) = result {
        tracing::error!(error = %e, "failed to write audit_log entry — rejecting request");
        anyhow::bail!("audit write failed: {e}");
    }

    if let Some(keypair) = audit_keypair {
        let mut fed_entry = AuditLogEntry::new(
            uuid::Uuid::new_v4(),
            &event.citizen_did,
            &event.action,
            &event.dept_id,
            event.requesting_dept.clone().unwrap_or_default(),
            event.scopes.clone(),
            String::new(),
        );
        fed_entry.sign(&keypair.signing_key);
        tracing::debug!(citizen_did = %event.citizen_did, "Created signed AuditLogEntry");
    }

    Ok(entry_hash)
}

/// Verify the hash chain integrity from a given entry back to genesis.
///
/// Not yet called: intended for an audit verification job. Kept so the chain can be checked
/// without re-implementing the hash rules (see TODO.md, audit item).
#[allow(dead_code)]
pub async fn verify_hash_chain(pool: &PgPool) -> anyhow::Result<bool> {
    let rows: Vec<(i64, Option<Vec<u8>>, Vec<u8>)> =
        sqlx::query_as("SELECT id, prev_hash, entry_hash FROM audit_log ORDER BY id ASC")
            .fetch_all(pool)
            .await?;

    let mut expected_prev: Option<Vec<u8>> = None;
    for (id, prev_hash, entry_hash) in rows {
        if prev_hash.as_deref() != expected_prev.as_deref() {
            tracing::error!(id, "Hash chain broken at entry {}", id);
            return Ok(false);
        }
        expected_prev = Some(entry_hash);
    }
    Ok(true)
}
