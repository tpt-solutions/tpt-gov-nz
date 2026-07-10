//! Per-department circuit breaker middleware.
//!
//! Protects the gateway (and callers) from a failing downstream department
//! service. After `failure_threshold` consecutive 5xx responses the breaker
//! for that department opens and further requests fail fast with `503` until a
//! cooldown elapses, at which point a single trial request is allowed
//! (half-open). A success closes the breaker; a failure re-opens it.
//!
//! Configure with:
//! - `TPT__GOV__CB_FAILURE_THRESHOLD` (default 5)
//! - `TPT__GOV__CB_COOLDOWN_SECS` (default 30)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::routes::extract_dept_id;
use crate::state::AppState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BreakerState {
    Closed,
    Open,
    HalfOpen,
}

struct Breaker {
    state: BreakerState,
    failures: u32,
    opened_at: Option<Instant>,
}

impl Default for Breaker {
    fn default() -> Self {
        Self {
            state: BreakerState::Closed,
            failures: 0,
            opened_at: None,
        }
    }
}

/// Registry of per-department circuit breakers.
#[derive(Clone)]
pub struct CircuitBreakerRegistry {
    breakers: Arc<Mutex<HashMap<String, Breaker>>>,
    failure_threshold: u32,
    cooldown: Duration,
}

impl CircuitBreakerRegistry {
    pub fn new(failure_threshold: u32, cooldown: Duration) -> Self {
        Self {
            breakers: Arc::new(Mutex::new(HashMap::new())),
            failure_threshold,
            cooldown,
        }
    }

    pub fn from_env() -> Self {
        let failure_threshold = std::env::var("TPT__GOV__CB_FAILURE_THRESHOLD")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);
        let cooldown_secs = std::env::var("TPT__GOV__CB_COOLDOWN_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);
        Self::new(failure_threshold, Duration::from_secs(cooldown_secs))
    }

    /// Returns `true` if a request to `dept_id` should be allowed through.
    ///
    /// Transitions an expired `Open` breaker to `HalfOpen` to permit one trial.
    pub fn allow(&self, dept_id: &str) -> bool {
        let now = Instant::now();
        let mut breakers = self.breakers.lock().unwrap();
        let b = breakers.entry(dept_id.to_string()).or_default();

        match b.state {
            BreakerState::Closed | BreakerState::HalfOpen => true,
            BreakerState::Open => {
                let elapsed = b
                    .opened_at
                    .map(|t| now.saturating_duration_since(t))
                    .unwrap_or_default();
                if elapsed >= self.cooldown {
                    b.state = BreakerState::HalfOpen;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Record a successful downstream response, closing the breaker.
    pub fn record_success(&self, dept_id: &str) {
        let mut breakers = self.breakers.lock().unwrap();
        let b = breakers.entry(dept_id.to_string()).or_default();
        b.state = BreakerState::Closed;
        b.failures = 0;
        b.opened_at = None;
    }

    /// Record a failed downstream response, possibly opening the breaker.
    pub fn record_failure(&self, dept_id: &str) {
        let now = Instant::now();
        let mut breakers = self.breakers.lock().unwrap();
        let b = breakers.entry(dept_id.to_string()).or_default();

        match b.state {
            BreakerState::HalfOpen => {
                // Trial request failed — re-open immediately.
                b.state = BreakerState::Open;
                b.opened_at = Some(now);
            }
            _ => {
                b.failures += 1;
                if b.failures >= self.failure_threshold {
                    b.state = BreakerState::Open;
                    b.opened_at = Some(now);
                }
            }
        }
    }
}

/// Axum middleware wrapping proxy routes with a per-department circuit breaker.
pub async fn circuit_breaker(State(app): State<AppState>, req: Request, next: Next) -> Response {
    let dept_id = extract_dept_id(req.uri().path());

    if let Some(dept) = &dept_id {
        if !app.breakers.allow(dept) {
            tracing::warn!(dept = %dept, "circuit open — failing fast");
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "error": "circuit_open",
                    "dept": dept,
                    "message": "downstream department is temporarily unavailable",
                })),
            )
                .into_response();
        }
    }

    let resp = next.run(req).await;

    if let Some(dept) = &dept_id {
        if resp.status().is_server_error() {
            app.breakers.record_failure(dept);
        } else {
            app.breakers.record_success(dept);
        }
    }

    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_after_threshold_failures() {
        let cb = CircuitBreakerRegistry::new(3, Duration::from_secs(30));
        assert!(cb.allow("ird"));
        cb.record_failure("ird");
        cb.record_failure("ird");
        assert!(cb.allow("ird")); // still closed after 2
        cb.record_failure("ird"); // 3rd -> open
        assert!(!cb.allow("ird"));
    }

    #[test]
    fn success_resets_failure_count() {
        let cb = CircuitBreakerRegistry::new(2, Duration::from_secs(30));
        cb.record_failure("ird");
        cb.record_success("ird");
        cb.record_failure("ird");
        // Only 1 failure since reset, so still closed.
        assert!(cb.allow("ird"));
    }

    #[test]
    fn half_open_after_cooldown_then_close_on_success() {
        let cb = CircuitBreakerRegistry::new(1, Duration::from_millis(10));
        cb.record_failure("ird"); // opens
        assert!(!cb.allow("ird"));
        std::thread::sleep(Duration::from_millis(15));
        assert!(cb.allow("ird")); // half-open trial allowed
        cb.record_success("ird");
        assert!(cb.allow("ird")); // closed again
    }

    #[test]
    fn half_open_reopens_on_failure() {
        let cb = CircuitBreakerRegistry::new(1, Duration::from_millis(10));
        cb.record_failure("ird"); // opens
        std::thread::sleep(Duration::from_millis(15));
        assert!(cb.allow("ird")); // half-open
        cb.record_failure("ird"); // trial fails -> reopen
        assert!(!cb.allow("ird"));
    }

    #[test]
    fn departments_are_independent() {
        let cb = CircuitBreakerRegistry::new(1, Duration::from_secs(30));
        cb.record_failure("ird");
        assert!(!cb.allow("ird"));
        assert!(cb.allow("winz"));
    }
}
