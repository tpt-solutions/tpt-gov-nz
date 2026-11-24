//! Liveness and readiness probes.
//!
//! `/health` (liveness) answers while the process serves requests. It never calls
//! downstream services, so an orchestrator does not restart the gateway just because a
//! department is down.
//!
//! `/ready` (readiness) probes the identity server and every registered department's
//! `/health` within the upstream timeout, using mTLS for departments when it is
//! configured. It answers 503 when any probe fails and names the failing targets, so
//! an operator can see which dependency is down. Error text from the network stack is
//! logged, not returned, so internal addresses do not leak to callers.

use axum::{
    Json,
    body::Bytes,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use http_body_util::Full;
use serde::Serialize;

use crate::routes::build_target;
use crate::state::AppState;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ProbeResult {
    pub name: String,
    pub ready: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    pub status: &'static str,
    pub checks: Vec<ProbeResult>,
}

/// Overall readiness: every target must be ready.
pub fn summarize(checks: &[ProbeResult]) -> (StatusCode, &'static str) {
    if checks.iter().all(|c| c.ready) {
        (StatusCode::OK, "ready")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "not_ready")
    }
}

fn failed(name: &str, error: &'static str) -> ProbeResult {
    ProbeResult {
        name: name.to_string(),
        ready: false,
        status: None,
        error: Some(error),
    }
}

/// Probe one target's `/health` endpoint. A 2xx answer means ready.
async fn probe(state: &AppState, name: &str, base_url: &str, use_mtls: bool) -> ProbeResult {
    let url = build_target(base_url, "health", None, use_mtls);
    let req = match Request::get(&url).body(Full::new(Bytes::new())) {
        Ok(req) => req,
        Err(e) => {
            tracing::error!(error = %e, target = name, "readiness probe URL is invalid");
            return failed(name, "invalid probe URL");
        }
    };

    let outcome = match (use_mtls, state.mtls.as_ref()) {
        // Fail closed: never fall back to plaintext when mTLS was requested.
        (true, None) => return failed(name, "mTLS client unavailable"),
        (true, Some(mtls)) => {
            match tokio::time::timeout(state.upstream_timeout, mtls.request(req)).await {
                Ok(Ok(resp)) => Ok(resp.status().as_u16()),
                Ok(Err(e)) => {
                    tracing::warn!(error = %e, target = name, "readiness probe (mTLS) failed");
                    Err("connection failed")
                }
                Err(_) => Err("timed out"),
            }
        }
        _ => match tokio::time::timeout(state.upstream_timeout, state.http.request(req)).await {
            Ok(Ok(resp)) => Ok(resp.status().as_u16()),
            Ok(Err(e)) => {
                tracing::warn!(error = %e, target = name, "readiness probe failed");
                Err("connection failed")
            }
            Err(_) => Err("timed out"),
        },
    };

    match outcome {
        Ok(status) if (200..300).contains(&status) => ProbeResult {
            name: name.to_string(),
            ready: true,
            status: Some(status),
            error: None,
        },
        Ok(status) => ProbeResult {
            name: name.to_string(),
            ready: false,
            status: Some(status),
            error: Some("unexpected status"),
        },
        Err(error) => failed(name, error),
    }
}

/// `GET /ready`: readiness, probing the identity server and every department in parallel.
pub async fn ready(State(state): State<AppState>) -> Response {
    let mut set = tokio::task::JoinSet::new();

    let identity_state = state.clone();
    let identity_url = state.identity_url.clone();
    set.spawn(async move { probe(&identity_state, "identity-server", &identity_url, false).await });

    for (dept_id, base_url) in state.registry.entries() {
        let dept_state = state.clone();
        let use_mtls = state.mtls.is_some();
        let name = format!("dept:{dept_id}");
        set.spawn(async move { probe(&dept_state, &name, &base_url, use_mtls).await });
    }

    let mut checks = Vec::new();
    while let Some(joined) = set.join_next().await {
        match joined {
            Ok(check) => checks.push(check),
            Err(e) => {
                tracing::error!(error = %e, "readiness probe task failed");
                checks.push(failed("probe-task", "probe task failed"));
            }
        }
    }
    checks.sort_by(|a, b| a.name.cmp(&b.name));
    for check in &checks {
        crate::metrics::record_dependency(&check.name, check.ready);
    }

    let (code, status) = summarize(&checks);
    (code, Json(ReadyResponse { status, checks })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(name: &str, ready: bool) -> ProbeResult {
        ProbeResult {
            name: name.into(),
            ready,
            status: None,
            error: None,
        }
    }

    #[test]
    fn all_ready_is_200() {
        let (code, status) = summarize(&[check("identity-server", true), check("dept:ird", true)]);
        assert_eq!(code, StatusCode::OK);
        assert_eq!(status, "ready");
    }

    #[test]
    fn any_failure_is_503() {
        let (code, status) = summarize(&[check("identity-server", true), check("dept:ird", false)]);
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(status, "not_ready");
    }

    #[test]
    fn failed_probe_reports_only_a_generic_error() {
        let json = serde_json::to_value(failed("dept:ird", "connection failed")).unwrap();
        assert_eq!(json["ready"], false);
        assert_eq!(json["error"], "connection failed");
        assert!(json.get("status").is_none());
    }
}
