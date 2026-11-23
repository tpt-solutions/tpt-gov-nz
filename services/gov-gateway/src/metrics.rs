//! Prometheus text exposition for the gateway.
//!
//! Exposes `gov_gateway_http_requests_total{method,status}`. Labels are deliberately
//! coarse: no request path, so cardinality stays bounded whatever callers send.

use std::collections::BTreeMap;
use std::fmt::Write;
use std::sync::{Mutex, MutexGuard};

use axum::{
    extract::Request,
    http::{Method, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

type Counts = BTreeMap<(&'static str, u16), u64>;

static REQUESTS: Mutex<Counts> = Mutex::new(BTreeMap::new());

/// Lock the counters. A poisoned lock still holds valid counts, so recover it rather than panic.
fn counts() -> MutexGuard<'static, Counts> {
    REQUESTS.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Map a method to a fixed label; anything unusual is folded into `OTHER`.
fn method_label(method: &Method) -> &'static str {
    match *method {
        Method::GET => "GET",
        Method::POST => "POST",
        Method::PUT => "PUT",
        Method::PATCH => "PATCH",
        Method::DELETE => "DELETE",
        Method::HEAD => "HEAD",
        Method::OPTIONS => "OPTIONS",
        _ => "OTHER",
    }
}

/// Count one response.
pub fn record_response(method: &Method, status: u16) {
    *counts().entry((method_label(method), status)).or_insert(0) += 1;
}

/// Middleware: count every response the gateway sends, including errors from inner layers.
pub async fn track(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let response = next.run(req).await;
    record_response(&method, response.status().as_u16());
    response
}

/// Render the counters in the Prometheus text format.
pub fn render() -> String {
    let mut out = String::from(
        "# HELP gov_gateway_http_requests_total HTTP responses sent by the gateway.\n\
         # TYPE gov_gateway_http_requests_total counter\n",
    );
    for ((method, status), count) in counts().iter() {
        // Writing to a String cannot fail.
        let _ = writeln!(
            out,
            "gov_gateway_http_requests_total{{method=\"{method}\",status=\"{status}\"}} {count}"
        );
    }
    out
}

/// `GET /metrics`
pub async fn endpoint() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], render())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_responses_by_method_and_status() {
        record_response(&Method::GET, 299);
        record_response(&Method::GET, 299);
        let text = render();
        assert!(text.contains("# TYPE gov_gateway_http_requests_total counter"));
        // Other tests share the global counters, so check for at least our two hits.
        let line = text
            .lines()
            .find(|l| l.starts_with("gov_gateway_http_requests_total{method=\"GET\",status=\"299\"}"))
            .expect("GET/299 series present");
        let count: u64 = line.rsplit(' ').next().unwrap().parse().unwrap();
        assert!(count >= 2);
    }

    #[test]
    fn unusual_methods_are_folded_into_other() {
        let method = Method::from_bytes(b"BREW").unwrap();
        record_response(&method, 418);
        assert!(render().contains("method=\"OTHER\",status=\"418\""));
    }

    #[test]
    fn render_never_includes_request_paths() {
        assert!(!render().contains("/v1/"));
    }
}
