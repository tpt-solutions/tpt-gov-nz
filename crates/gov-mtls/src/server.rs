//! TLS serving with client-certificate authentication for internal services.
//!
//! [`serve`] accepts TCP connections, completes a TLS handshake that requires a
//! certificate issued by the internal CA, and then serves an axum [`Router`] over
//! the connection. Every request handled on such a connection carries a
//! [`PeerCertificate`] extension, which holds the SHA-256 fingerprint of the
//! client's leaf certificate. Handlers and middleware use that fingerprint to
//! decide which peer is calling (for example, the gateway or the staff portal).
//!
//! Connections that fail the handshake never reach the router.

use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Result;
use axum::{
    Extension, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto,
    service::TowerToHyperService,
};
use rustls::ServerConfig;
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

/// Identity of the client that completed the mTLS handshake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerCertificate {
    /// SHA-256 of the client's leaf certificate (DER), lowercase hex.
    pub fingerprint_sha256: String,
}

/// SHA-256 of a DER-encoded certificate, as lowercase hex.
pub fn fingerprint_sha256(der: &[u8]) -> String {
    Sha256::digest(der)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Serve `app` over TLS on `listener`. The server config must require client
/// certificates (see [`crate::server_config`]).
///
/// Runs until the listener fails. Per-connection errors are logged and do not
/// stop the accept loop.
pub async fn serve(listener: TcpListener, app: Router, config: Arc<ServerConfig>) -> Result<()> {
    crate::install_crypto_provider();
    let acceptor = TlsAcceptor::from(config);
    loop {
        let (tcp, peer_addr) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                tracing::warn!(error = %e, "accept failed; retrying");
                continue;
            }
        };

        let acceptor = acceptor.clone();
        let app = app.clone();
        tokio::spawn(async move {
            let tls = match acceptor.accept(tcp).await {
                Ok(tls) => tls,
                Err(e) => {
                    tracing::debug!(error = %e, peer = %peer_addr, "mTLS handshake rejected");
                    return;
                }
            };

            // The verifier requires a client certificate, so this is present.
            // If it is somehow absent, refuse the connection rather than serve it.
            let leaf = tls
                .get_ref()
                .1
                .peer_certificates()
                .and_then(|certs| certs.first());
            let Some(leaf) = leaf else {
                tracing::warn!(peer = %peer_addr, "mTLS connection without a client certificate");
                return;
            };
            let peer = PeerCertificate {
                fingerprint_sha256: fingerprint_sha256(leaf.as_ref()),
            };

            let app = app.layer(Extension(peer));
            let service = TowerToHyperService::new(app);
            if let Err(e) = auto::Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(tls), service)
                .await
            {
                tracing::debug!(error = %e, peer = %peer_addr, "mTLS connection closed");
            }
        });
    }
}

/// How a service listens, decided from its environment by [`transport_from_env`].
pub enum Transport {
    /// Serve over mTLS. Only the listed client certificates may call.
    Mtls {
        config: Arc<ServerConfig>,
        allowed: HashSet<String>,
    },
    /// Plain HTTP, for local development only (`TPT__GOV__ENV=development`).
    Development,
}

/// Choose the transport from the environment.
///
/// - With `TPT__GOV__MTLS_CERT`, `TPT__GOV__MTLS_KEY` and `TPT__GOV__MTLS_CA` set,
///   the service serves over mTLS. `TPT__GOV__MTLS_ALLOWED_FINGERPRINTS` must list at
///   least one client certificate fingerprint (comma-separated SHA-256 hex).
/// - Without them, the service refuses to start unless `TPT__GOV__ENV=development`.
pub fn transport_from_env() -> Result<Transport> {
    if let Some(paths) = crate::TlsPaths::from_env() {
        let config = Arc::new(crate::server_config(&paths)?);
        let raw = std::env::var("TPT__GOV__MTLS_ALLOWED_FINGERPRINTS").unwrap_or_default();
        let allowed = parse_fingerprint_list(&raw)?;
        if allowed.is_empty() {
            anyhow::bail!("TPT__GOV__MTLS_ALLOWED_FINGERPRINTS must list at least one peer");
        }
        return Ok(Transport::Mtls { config, allowed });
    }

    if std::env::var("TPT__GOV__ENV").as_deref() == Ok("development") {
        return Ok(Transport::Development);
    }

    anyhow::bail!(
        "mTLS is required: set TPT__GOV__MTLS_CERT, TPT__GOV__MTLS_KEY and TPT__GOV__MTLS_CA, \
         or TPT__GOV__ENV=development for local plain-HTTP runs"
    )
}

/// Parse a comma-separated list of SHA-256 fingerprints (64 hex characters each).
/// Input is case-insensitive; the result is lowercase. Blank entries are ignored.
pub fn parse_fingerprint_list(raw: &str) -> Result<HashSet<String>> {
    let mut out = HashSet::new();
    for entry in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let fp = entry.to_ascii_lowercase();
        if fp.len() != 64 || !fp.bytes().all(|b| b.is_ascii_hexdigit()) {
            anyhow::bail!("not a SHA-256 fingerprint: {entry}");
        }
        out.insert(fp);
    }
    Ok(out)
}

/// Axum middleware: refuse any request whose client certificate is not in the allow-list.
/// A request with no certificate (which [`serve`] never passes) is refused too.
pub async fn require_allowed_peer(
    State(allowed): State<Arc<HashSet<String>>>,
    peer: Option<Extension<PeerCertificate>>,
    req: Request,
    next: Next,
) -> Response {
    match peer {
        Some(Extension(p)) if allowed.contains(&p.fingerprint_sha256) => next.run(req).await,
        _ => StatusCode::FORBIDDEN.into_response(),
    }
}

/// Wrap a router so only allow-listed client certificates reach its routes.
pub fn restrict_to_peers(app: Router, allowed: HashSet<String>) -> Router {
    app.layer(middleware::from_fn_with_state(
        Arc::new(allowed),
        require_allowed_peer,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_stable_lowercase_hex() {
        // SHA-256 of the empty input.
        assert_eq!(
            fingerprint_sha256(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
