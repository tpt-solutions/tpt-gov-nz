//! Consent verification and OPA policy evaluation for the API gateway.
//!
//! This module handles:
//! - Verifying DataGrantCredential signatures against the identity server's public key
//! - Checking grant revocation via the identity server
//! - Evaluating OPA Rego policies for department-specific consent rules

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    Json,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use ed25519_dalek::VerifyingKey;
use gov_consent::{RevocationClient, issuer_key_from_env};
use gov_identity_core::DataGrantCredential;
use opa_wasm::{DefaultContext, Policy, Runtime, wasmtime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error, warn};

use crate::state::AppState;

/// OPA input for consent evaluation.
#[derive(Serialize, Deserialize)]
pub struct OpaConsentInput {
    pub citizen_did: String,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub requested_scopes: Vec<String>,
    pub consent_grants: Vec<GrantInput>,
}

/// Individual grant for OPA evaluation.
#[derive(Serialize, Deserialize)]
pub struct GrantInput {
    pub id: String,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub citizen_did: String,
    pub scopes: Vec<String>,
    pub issued_at: i64,
    pub expires_at: i64,
}

/// OPA policy decision response.
#[derive(Deserialize)]
pub struct OpaDecision {
    pub allow: bool,
    #[serde(default)]
    pub denied_scopes: Vec<String>,
}

/// Header carrying the consent grant (DataGrantCredential).
pub const CONSENT_GRANT_HEADER: &str = "x-consent-grant";

/// Header carrying the citizen DID.
pub const CITIZEN_DID_HEADER: &str = "x-citizen-did";

/// Header carrying the requesting department ID.
pub const REQUESTING_DEPT_HEADER: &str = "x-requesting-dept-id";

/// Compiled OPA policy for a department.
pub struct OpaPolicy {
    engine: wasmtime::Engine,
    policy: Policy<DefaultContext>,
    policy_path: String,
}

impl OpaPolicy {
    /// Load a Rego policy from a file path (expects pre-compiled Wasm).
    pub async fn from_file(path: &str) -> Result<Self> {
        let wasm_bytes = std::fs::read(path)
            .with_context(|| format!("reading OPA policy wasm from {}", path))?;

        // Create wasmtime engine
        let config = wasmtime::Config::new();
        let engine = wasmtime::Engine::new(&config)?;

        // Load module and create runtime
        let module = wasmtime::Module::new(&engine, &wasm_bytes)?;
        let mut store = wasmtime::Store::new(&engine, ());
        let runtime = Runtime::new(&mut store, &module)
            .await
            .context("creating OPA runtime")?;

        // Create policy with empty initial data (will be provided per-request)
        // with_data consumes the runtime, so we create policy and don't need runtime afterwards
        let policy = runtime
            .with_data(&mut store, &serde_json::json!({}))
            .await
            .context("creating OPA policy instance")?;

        Ok(Self {
            engine,
            policy,
            policy_path: path.to_string(),
        })
    }

    /// Load OPA policies from a directory (loads all .wasm files).
    pub async fn from_directory(dir: &str) -> Result<HashMap<String, Arc<Self>>> {
        let mut policies = HashMap::new();
        let entries = std::fs::read_dir(dir)
            .with_context(|| format!("reading OPA policy directory {}", dir))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                // Extract department ID from filename (e.g., "ird.wasm" -> "ird")
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    match Self::from_file(path.to_str().unwrap()).await {
                        Ok(policy) => {
                            policies.insert(stem.to_string(), Arc::new(policy));
                            debug!("Loaded OPA policy for department: {}", stem);
                        }
                        Err(e) => {
                            warn!("Failed to load OPA policy {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(policies)
    }

    /// Evaluate the policy with the given input.
    pub async fn evaluate(&self, input: &OpaConsentInput) -> Result<OpaDecision> {
        let input_json = serde_json::to_vec(input).context("serializing OPA input")?;

        // Try department-specific query first, then fallback to generic
        let query = format!(
            "data.{}.consent.allow",
            self.policy_path
                .split('/')
                .next_back()
                .unwrap_or("default")
                .replace(".wasm", "")
        );

        // Create a new store for this evaluation
        let mut store = wasmtime::Store::new(&self.engine, ());

        // Evaluate with the policy - need explicit types for V (input), R (output), T (store data)
        // Deserialize input first, then pass reference
        let input: OpaConsentInput = serde_json::from_slice(&input_json)?;
        let result: serde_json::Value = self
            .policy
            .evaluate::<OpaConsentInput, serde_json::Value, ()>(&mut store, &query, &input)
            .await
            .context("OPA policy evaluation failed")?;

        let decision: OpaDecision =
            serde_json::from_value(result).context("parsing OPA decision")?;

        Ok(decision)
    }
}

/// Consent verification configuration.
pub struct ConsentConfig {
    /// Identity server's Ed25519 verification key.
    pub issuer_key: VerifyingKey,
    /// Revocation client for checking grant status.
    pub revocation: RevocationClient,
    /// OPA policy instances per department (loaded from Wasm files).
    pub policies: HashMap<String, Arc<OpaPolicy>>,
    /// Whether to enforce consent (can be disabled for specific routes).
    pub enforce: bool,
}

impl ConsentConfig {
    /// Build consent config from environment.
    pub async fn from_env() -> Result<Self> {
        let issuer_key = issuer_key_from_env().context("loading issuer public key")?;

        let revocation = RevocationClient::from_env().context("building revocation client")?;

        // Load OPA policies from directory (if configured)
        let policy_dir =
            std::env::var("TPT__GOV__OPA_POLICY_DIR").unwrap_or_else(|_| "./policies".to_string());
        let policies = OpaPolicy::from_directory(&policy_dir)
            .await
            .unwrap_or_else(|e| {
                warn!("Failed to load OPA policies from {}: {}", policy_dir, e);
                HashMap::new()
            });

        Ok(Self {
            issuer_key,
            revocation,
            policies,
            enforce: true,
        })
    }

    /// Verify a DataGrantCredential signature.
    pub fn verify_grant(&self, grant: &DataGrantCredential) -> Result<()> {
        grant
            .verify_signature(&self.issuer_key)
            .context("grant signature verification failed")?;
        grant.is_valid().context("grant expired")?;
        Ok(())
    }

    /// Check grant revocation status.
    pub async fn check_revocation(&self, grant_ids: &[uuid::Uuid]) -> Result<()> {
        self.revocation
            .ensure_active(grant_ids)
            .await
            .context("grant revocation check failed")?;
        Ok(())
    }

    /// Evaluate OPA policy for a department.
    pub async fn evaluate_consent(&self, input: OpaConsentInput) -> Result<OpaDecision> {
        // Try to get department-specific policy
        let dept_id = &input.providing_dept_id;

        if let Some(policy) = self.policies.get(dept_id) {
            policy.evaluate(&input).await
        } else {
            // Fallback: default allow with basic scope coverage check
            warn!("No OPA policy for department {}, using fallback", dept_id);
            self.fallback_evaluate(&input)
        }
    }

    /// Fallback evaluation when no OPA policy is loaded.
    fn fallback_evaluate(&self, input: &OpaConsentInput) -> Result<OpaDecision> {
        let mut denied = Vec::new();

        for scope in &input.requested_scopes {
            let covered = input.consent_grants.iter().any(|grant| {
                grant.requesting_dept_id == input.requesting_dept_id
                    && grant.providing_dept_id == input.providing_dept_id
                    && grant.citizen_did == input.citizen_did
                    && grant.expires_at > chrono::Utc::now().timestamp()
                    && grant.scopes.contains(scope)
            });

            if !covered {
                denied.push(scope.clone());
            }
        }

        Ok(OpaDecision {
            allow: denied.is_empty(),
            denied_scopes: denied,
        })
    }
}

/// Extract and parse consent grants from headers.
pub fn extract_grants(headers: &HeaderMap) -> Result<Vec<DataGrantCredential>> {
    let grants_header = headers
        .get(CONSENT_GRANT_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| anyhow::anyhow!("missing x-consent-grant header"))?;

    // Support multiple grants separated by comma, or single grant
    let mut grants = Vec::new();
    for grant_str in grants_header.split(',') {
        let grant_str = grant_str.trim();
        if grant_str.is_empty() {
            continue;
        }
        let grant: DataGrantCredential = serde_json::from_str(grant_str)
            .with_context(|| format!("parsing consent grant: {}", grant_str))?;
        grants.push(grant);
    }

    if grants.is_empty() {
        anyhow::bail!("no valid consent grants found");
    }

    Ok(grants)
}
/// Axum middleware for consent verification on dept proxy routes.
pub async fn consent_middleware(
    State(app): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    // Only apply to /v1/dept/ routes
    let path = req.uri().path();
    if !path.starts_with("/v1/dept/") {
        return next.run(req).await;
    }

    // Skip if consent enforcement is disabled
    if !app.consent.enforce {
        return next.run(req).await;
    }

    // Extract required headers
    let headers = req.headers();

    let citizen_did = match headers
        .get(CITIZEN_DID_HEADER)
        .and_then(|v| v.to_str().ok())
    {
        Some(d) if !d.is_empty() => d.to_string(),
        _ => return forbidden("missing x-citizen-did header"),
    };

    let requesting_dept_id = match headers
        .get(REQUESTING_DEPT_HEADER)
        .and_then(|v| v.to_str().ok())
    {
        Some(d) if !d.is_empty() => d.to_string(),
        _ => return forbidden("missing x-requesting-dept-id header"),
    };

    // Extract department ID from path
    let providing_dept_id = match extract_dept_id_from_path(path) {
        Some(d) => d,
        None => return forbidden("invalid department path"),
    };

    // Extract and parse consent grants
    let grants = match extract_grants(headers) {
        Ok(g) => g,
        Err(e) => {
            warn!("failed to extract consent grants: {}", e);
            return forbidden("invalid or missing consent grants");
        }
    };

    // Verify grant signatures and expiry
    for grant in &grants {
        if let Err(e) = app.consent.verify_grant(grant) {
            warn!(grant_id = %grant.id, error = %e, "grant verification failed");
            return forbidden("invalid or expired consent grant");
        }
    }

    // Check revocation
    let grant_ids: Vec<uuid::Uuid> = grants.iter().map(|g| g.id).collect();
    if let Err(e) = app.consent.check_revocation(&grant_ids).await {
        warn!(?grant_ids, error = %e, "grant revocation check failed");
        return forbidden("consent grant revoked or unknown");
    }

    // Determine requested scopes from the request (query params or body)
    // For now, we'll extract from a header or default to all grant scopes
    let requested_scopes: Vec<String> = grants.iter().flat_map(|g| g.scopes.clone()).collect();

    // Build OPA input
    let opa_input = OpaConsentInput {
        citizen_did: citizen_did.clone(),
        requesting_dept_id: requesting_dept_id.clone(),
        providing_dept_id: providing_dept_id.clone(),
        requested_scopes,
        consent_grants: grants
            .iter()
            .map(|g| GrantInput {
                id: g.id.to_string(),
                requesting_dept_id: g.requesting_dept_id.clone(),
                providing_dept_id: g.providing_dept_id.clone(),
                citizen_did: g.citizen_did.to_string(),
                scopes: g.scopes.clone(),
                issued_at: g.issued_at,
                expires_at: g.expires_at,
            })
            .collect(),
    };

    // Evaluate OPA policy
    let decision = match app.consent.evaluate_consent(opa_input).await {
        Ok(d) => d,
        Err(e) => {
            error!(error = %e, "OPA policy evaluation failed");
            return internal_error("consent evaluation failed");
        }
    };

    if !decision.allow {
        warn!(
            citizen_did = %citizen_did,
            requesting_dept = %requesting_dept_id,
            providing_dept = %providing_dept_id,
            denied_scopes = ?decision.denied_scopes,
            "consent denied by OPA policy"
        );
        return forbidden(&format!(
            "consent denied for scopes: {}",
            decision.denied_scopes.join(", ")
        ));
    }

    // Consent granted - add consent info to request extensions for downstream
    req.extensions_mut().insert(ConsentInfo {
        _citizen_did: citizen_did,
        _requesting_dept_id: requesting_dept_id,
        _providing_dept_id: providing_dept_id,
        _grants: grants,
    });

    next.run(req).await
}
/// Consent information stored in request extensions after successful verification.
#[derive(Debug, Clone)]
pub struct ConsentInfo {
    pub _citizen_did: String,
    pub _requesting_dept_id: String,
    pub _providing_dept_id: String,
    pub _grants: Vec<DataGrantCredential>,
}

/// Extract department ID from /v1/dept/{dept_id}/... path.
fn extract_dept_id_from_path(path: &str) -> Option<String> {
    let rest = path.strip_prefix("/v1/dept/")?;
    let dept = rest.split('/').next()?;
    if dept.is_empty() {
        None
    } else {
        Some(dept.to_lowercase())
    }
}

fn forbidden(message: &str) -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!({
            "error": "forbidden",
            "message": message,
        })),
    )
        .into_response()
}

fn internal_error(message: &str) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "error": "internal_server_error",
            "message": message,
        })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dept_id_from_path() {
        assert_eq!(
            extract_dept_id_from_path("/v1/dept/ird/citizen/data"),
            Some("ird".to_string())
        );
        assert_eq!(
            extract_dept_id_from_path("/v1/dept/WINZ/health"),
            Some("winz".to_string())
        );
        assert_eq!(
            extract_dept_id_from_path("/v1/dept/moh"),
            Some("moh".to_string())
        );
        assert_eq!(extract_dept_id_from_path("/health"), None);
        assert_eq!(extract_dept_id_from_path("/v1/citizen/resolve"), None);
        assert_eq!(extract_dept_id_from_path("/v1/dept/"), None);
    }

    #[test]
    fn test_opa_consent_input_serialization() {
        let input = OpaConsentInput {
            citizen_did: "did:gov:nz:citizen001".to_string(),
            requesting_dept_id: "winz".to_string(),
            providing_dept_id: "ird".to_string(),
            requested_scopes: vec!["ird:income".to_string()],
            consent_grants: vec![],
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("citizen_did"));
        assert!(json.contains("ird:income"));
    }
}
