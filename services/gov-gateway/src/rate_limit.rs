//! Per-caller rate limiting middleware.
//!
//! Uses a fixed-window counter keyed by the authenticated caller (the JWT
//! `sub`, i.e. the citizen DID). Each caller gets an independent budget so a
//! burst from one citizen cannot starve everyone else.
//!
//! Configure with:
//! - `TPT__GOV__RATE_LIMIT_RPM` — requests per minute per caller (default 120)
//! - `TPT__GOV__RATE_LIMIT_MAX_CALLERS` — maximum distinct callers tracked
//!   (default 10 000). The map is bounded to avoid unbounded memory growth from
//!   many distinct (possibly forged) caller ids.

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

use crate::auth::Claims;
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
    max_callers: usize,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self::with_max_callers(max_requests, window, 10_000)
    }

    pub fn with_max_callers(max_requests: u32, window: Duration, max_callers: usize) -> Self {
        Self {
            windows: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
            max_callers: max_callers.max(1),
        }
    }

    pub fn from_env() -> Self {
        let max_requests = std::env::var("TPT__GOV__RATE_LIMIT_RPM")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(120);
        let max_callers = std::env::var("TPT__GOV__RATE_LIMIT_MAX_CALLERS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10_000);
        Self::with_max_callers(max_requests, Duration::from_secs(60), max_callers)
    }

    /// Lock the window map, recovering from a poisoned mutex.
    ///
    /// A panic while holding the lock cannot leave the counters in a
    /// memory-unsafe state; recovering keeps the gateway serving instead of
    /// panicking on every subsequent request.
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<String, Window>> {
        self.windows
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Drop expired windows, then evict the window closest to expiry if the map
    /// is still at capacity. Keeps memory bounded without a background task.
    fn make_room(windows: &mut HashMap<String, Window>, now: Instant, max_callers: usize) {
        if windows.len() < max_callers {
            return;
        }
        windows.retain(|_, w| now < w.reset_at);
        if windows.len() < max_callers {
            return;
        }
        if let Some(key) = windows
            .iter()
            .min_by_key(|(_, w)| w.reset_at)
            .map(|(k, _)| k.clone())
        {
            windows.remove(&key);
        }
    }

    /// Record a request for `caller`.
    ///
    /// Returns `Ok(remaining)` when allowed, or `Err(retry_after_secs)` when the
    /// caller's window budget is exhausted.
    pub fn check(&self, caller: &str) -> Result<u32, u64> {
        let now = Instant::now();
        let mut windows = self.lock();

        if !windows.contains_key(caller) {
            Self::make_room(&mut windows, now, self.max_callers);
        }

        let w = windows.entry(caller.to_string()).or_insert_with(|| Window {
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

    /// Number of callers currently tracked (for tests / metrics).
    pub fn tracked_callers(&self) -> usize {
        self.lock().len()
    }
}

/// Axum middleware applying per-caller rate limits to protected routes.
pub async fn rate_limit(State(app): State<AppState>, req: Request, next: Next) -> Response {
    // JWT auth runs before this middleware, so an authenticated caller id is
    // always present. Fall back to a single shared bucket rather than trusting
    // the (spoofable) `x-forwarded-for` header.
    let caller = req
        .extensions()
        .get::<Claims>()
        .map(|c| c.sub.clone())
        .unwrap_or_else(|| "unauthenticated".to_string());

    match app.rate_limiter.check(&caller) {
        Ok(remaining) => {
            let mut resp = next.run(req).await;
            if let Ok(val) = remaining.to_string().parse() {
                resp.headers_mut().insert("x-ratelimit-remaining", val);
            }
            resp
        }
        Err(retry_after) => {
            tracing::warn!(caller = %caller, retry_after, "caller rate limit exceeded");
            (
                StatusCode::TOO_MANY_REQUESTS,
                [("retry-after", retry_after.to_string())],
                Json(json!({
                    "error": "rate_limited",
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
        assert!(limiter.check("did:gov:nz:a").is_ok());
        assert!(limiter.check("did:gov:nz:a").is_ok());
        assert!(limiter.check("did:gov:nz:a").is_ok());
        assert!(limiter.check("did:gov:nz:a").is_err());
    }

    #[test]
    fn callers_have_independent_budgets() {
        let limiter = RateLimiter::new(1, Duration::from_secs(60));
        assert!(limiter.check("did:gov:nz:a").is_ok());
        assert!(limiter.check("did:gov:nz:a").is_err());
        // Another caller still has their full budget.
        assert!(limiter.check("did:gov:nz:b").is_ok());
    }

    #[test]
    fn window_resets_after_expiry() {
        let limiter = RateLimiter::new(1, Duration::from_millis(10));
        assert!(limiter.check("did:gov:nz:a").is_ok());
        assert!(limiter.check("did:gov:nz:a").is_err());
        std::thread::sleep(Duration::from_millis(15));
        assert!(limiter.check("did:gov:nz:a").is_ok());
    }

    #[test]
    fn map_is_bounded() {
        let limiter = RateLimiter::with_max_callers(10, Duration::from_secs(60), 3);
        for i in 0..20 {
            let _ = limiter.check(&format!("caller-{i}"));
        }
        assert!(limiter.tracked_callers() <= 3);
    }
}
