//! Optional OPA sidecar client for the generic department node.
//!
//! When a department node is deployed with an OPA sidecar (`OPA_URL` set) and
//! built with the `opa` feature, the route layer defers the consent decision to
//! the deployed policy (e.g. `policies/ird.rego`). The local [`crate::consent`]
//! evaluation is always available as a fallback, so the service functions
//! without OPA (and in tests).

use crate::consent::ConsentInput;

#[cfg(feature = "opa")]
use gov_identity_core::DataGrantCredential;
#[cfg(feature = "opa")]
use serde::Serialize;

#[cfg(feature = "opa")]
#[derive(Serialize)]
struct OpaInput<'a> {
    requesting_dept_id: &'a str,
    providing_dept_id: String,
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

/// Query the OPA sidecar for the `<dept>.consent` decision.
///
/// Returns `None` when OPA is not configured or unavailable (caller falls back
/// to the local [`crate::consent`] evaluation). Only compiled with the `opa`
/// feature.
#[cfg(feature = "opa")]
pub async fn evaluate(input: &ConsentInput<'_>) -> Option<bool> {
    let base = std::env::var("OPA_URL").ok()?;
    let client = reqwest::Client::new();

    let providing = crate::consent::providing_dept_id();
    let url = format!("{base}/v1/data/{providing}/consent");
    let body = OpaInput {
        requesting_dept_id: input.requesting_dept_id,
        providing_dept_id: providing,
        citizen_did: input.citizen_did,
        requested_scopes: input.requested_scopes,
        consent_grants: input.consent_grants,
    };

    let resp = client
        .post(&url)
        .json(&serde_json::json!({ "input": body }))
        .send()
        .await
        .ok()?;

    let parsed: OpaResponse = resp.json().await.ok()?;
    Some(parsed.result.allow)
}

/// Fallback used when the `opa` feature is disabled — always defer to local logic.
#[cfg(not(feature = "opa"))]
pub async fn evaluate(_input: &ConsentInput<'_>) -> Option<bool> {
    None
}
