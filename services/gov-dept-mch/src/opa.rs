//! Optional OPA sidecar client for the Ministry for Culture and Heritage node.

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
        providing_dept_id: "mch",
        citizen_did: input.citizen_did,
        requested_scopes: input.requested_scopes,
        consent_grants: input.consent_grants,
    };

    let url = format!("{base}/v1/data/mch/consent");
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
