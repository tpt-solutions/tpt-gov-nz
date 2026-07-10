//! Per-department rate limiting middleware.
//!
//! Uses a simple fixed-window counter keyed by department id. Each department
//! gets an independent budget so a burst against one dept does not starve the
//! others. Configure with `TPT__GOV__RATE_LIMIT_RPM` (requests per minute,
//! default 120).

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

struct Window {
    count: u32,
    reset_at: Instant,
}

/// Fixed-window rate limiter, shared across handler invocations.
#[derive(Clone)]
pub struct RateLimiter {
    windows: Arc<Mutex<HashMap<String, Window>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            windows: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub fn from_env() -> Self {
        let max_requests = std::env::var("TPT__GOV__RATE_LIMIT_RPM")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(120);
        Self::new(max_requests, Duration::from_secs(60))
    }

    /// Record a request for `dept_id`.
    ///
    /// Returns `Ok(remaining)` when allowed, or `Err(retry_after_secs)` when the
    /// department's window budget is exhausted.
    pub fn check(&self, dept_id: &str) -> Result<u32, u64> {
        let now = Instant::now();
        let mut windows = self.windows.lock().unwrap();
        let w = windows.entry(dept_id.to_string()).or_insert_with(|| Window {
            count: 0,
            reset_at: now + self.window,
        });

        if now >= w.reset_at {
            w.count = 0;
            w.reset_at = now + self.window;
        }

        if w.count >= self.max_requests {
            let retry = w.reset_at.saturating_duration_since(now).as_secs() + 1;
            Err(retry)
        } else {
            w.count += 1;
            Ok(self.max_requests - w.count)
        }
    }
}

/// Axum middleware applying per-department rate limits to proxy routes.
pub async fn rate_limit(State(app): State<AppState>, req: Request, next: Next) -> Response {
    let Some(dept_id) = extract_dept_id(req.uri().path()) else {
        return next.run(req).await;
    };

    match app.rate_limiter.check(&dept_id) {
        Ok(remaining) => {
            let mut resp = next.run(req).await;
            if let Ok(val) = remaining.to_string().parse() {
                resp.headers_mut().insert("x-ratelimit-remaining", val);
            }
            resp
        }
        Err(retry_after) => {
            tracing::warn!(dept = %dept_id, retry_after, "department rate limit exceeded");
            (
                StatusCode::TOO_MANY_REQUESTS,
                [("retry-after", retry_after.to_string())],
                Json(json!({
                    "error": "rate_limited",
                    "dept": dept_id,
                    "retry_after": retry_after,
                })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_up_to_limit_then_blocks() {
        let limiter = RateLimiter::new(3, Duration::from_secs(60));
        assert!(limiter.check("ird").is_ok());
        assert!(limiter.check("ird").is_ok());
        assert!(limiter.check("ird").is_ok());
        assert!(limiter.check("ird").is_err());
    }

    #[test]
    fn departments_have_independent_budgets() {
        let limiter = RateLimiter::new(1, Duration::from_secs(60));
        assert!(limiter.check("ird").is_ok());
        assert!(limiter.check("ird").is_err());
        // winz still has its full budget.
        assert!(limiter.check("winz").is_ok());
    }

    #[test]
    fn window_resets_after_expiry() {
        let limiter = RateLimiter::new(1, Duration::from_millis(10));
        assert!(limiter.check("ird").is_ok());
        assert!(limiter.check("ird").is_err());
        std::thread::sleep(Duration::from_millis(15));
        assert!(limiter.check("ird").is_ok());
    }
}
