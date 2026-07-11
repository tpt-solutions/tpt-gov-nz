//! Mutual TLS for tpt-gov-nz internal services.
//!
//! Every internal service (gateway, identity server, department nodes, federation
//! node) should authenticate its peers with mutual TLS: each service presents a
//! certificate signed by a private internal CA, and verifies the peer's
//! certificate against that CA. This module loads PEM material into rustls
//! configurations and provides helpers to (a) build server configs (which
//! require a valid client certificate) and (b) issue requests to peers over mTLS.
//!
//! Certificates are generated with `scripts/gen-mtls-certs.sh` (see repo root).
//! Enable per service via env vars (see [`TlsPaths::from_env`]).

use std::fs;
use std::io::BufRead;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use base64::Engine;
use bytes::Bytes;
use http::Uri;
use http_body_util::Full;
use hyper::Request;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
use rustls::{ClientConfig, RootCertStore, ServerConfig};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

/// Filesystem locations of the PEM material for one service's mTLS identity.
#[derive(Clone, Debug)]
pub struct TlsPaths {
    /// Service (or client) certificate, PEM, end-entity first.
    pub cert: PathBuf,
    /// Service (or client) private key, PEM (PKCS#8).
    pub key: PathBuf,
    /// Internal CA certificate(s), PEM, used to verify peers.
    pub ca: PathBuf,
}

impl TlsPaths {
    /// Read mTLS material locations from the environment.
    ///
    /// | Var | Meaning |
    /// | --- | --- |
    /// | `TPT__GOV__MTLS_CERT` | this service's certificate (PEM) |
    /// | `TPT__GOV__MTLS_KEY`  | this service's private key (PEM, PKCS#8) |
    /// | `TPT__GOV__MTLS_CA`   | internal CA certificate(s) (PEM) |
    ///
    /// Returns `None` when any variable is missing, i.e. mTLS is opt-in.
    pub fn from_env() -> Option<TlsPaths> {
        let cert = std::env::var("TPT__GOV__MTLS_CERT").ok()?;
        let key = std::env::var("TPT__GOV__MTLS_KEY").ok()?;
        let ca = std::env::var("TPT__GOV__MTLS_CA").ok()?;
        Some(TlsPaths {
            cert: PathBuf::from(cert),
            key: PathBuf::from(key),
            ca: PathBuf::from(ca),
        })
    }
}

/// Decode all `-----BEGIN ...-----` / `-----END ...-----` PEM blocks in a file.
fn read_pem_blocks(path: &PathBuf) -> Result<Vec<Vec<u8>>> {
    let file = fs::File::open(path)
        .with_context(|| format!("opening PEM file {}", path.display()))?;
    let reader = std::io::BufReader::new(file);
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let mut buf: Option<String> = None;
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.starts_with("-----BEGIN ") {
            buf = Some(String::new());
        } else if trimmed.starts_with("-----END ") {
            if let Some(b64) = buf.take() {
                let der = base64::engine::general_purpose::STANDARD
                    .decode(b64.trim())
                    .with_context(|| format!("base64-decoding PEM block in {}", path.display()))?;
                blocks.push(der);
            }
        } else if let Some(b) = buf.as_mut() {
            b.push_str(trimmed);
        }
    }
    Ok(blocks)
}

fn load_certs(path: &PathBuf) -> Result<Vec<CertificateDer<'static>>> {
    let blocks = read_pem_blocks(path)?;
    Ok(blocks.into_iter().map(CertificateDer::from).collect())
}

fn load_key(path: &PathBuf) -> Result<PrivateKeyDer<'static>> {
    let blocks = read_pem_blocks(path)?;
    let der = blocks
        .into_iter()
        .next()
        .context("no private key block found")?;
    Ok(PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(der)))
}

fn ca_store(path: &PathBuf) -> Result<RootCertStore> {
    let certs = load_certs(path)?;
    let mut store = RootCertStore::empty();
    for c in certs {
        store.add(c).context("adding CA certificate to root store")?;
    }
    Ok(store)
}

/// Build a rustls [`ServerConfig`] that **requires** a valid client certificate
/// issued by the internal CA. Used by every internal service's listener.
pub fn server_config(paths: &TlsPaths) -> Result<ServerConfig> {
    let certs = load_certs(&paths.cert)?;
    let key = load_key(&paths.key)?;
    let roots = ca_store(&paths.ca)?;

    let verifier = rustls::server::WebPkiClientVerifier::builder(Arc::new(roots))
        .build()
        .context("building mTLS client-certificate verifier")?;

    ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(certs, key)
        .context("building mTLS server config")
}

/// Build a rustls [`ClientConfig`] that presents this service's client
/// certificate and verifies the peer against the internal CA.
pub fn client_config(paths: &TlsPaths) -> Result<ClientConfig> {
    let certs = load_certs(&paths.cert)?;
    let key = load_key(&paths.key)?;
    let roots = ca_store(&paths.ca)?;

    ClientConfig::builder()
        .with_root_certificates(roots)
        .with_client_auth_cert(certs, key)
        .context("building mTLS client config")
}

/// An mTLS-aware HTTP client for calling internal peers.
///
/// Built from [`client_config`]; issues requests over TLS presenting this
/// service's client certificate. The peer's certificate is verified against the
/// internal CA by the rustls client config.
#[derive(Clone)]
pub struct MtlsClient {
    config: Arc<ClientConfig>,
}

impl MtlsClient {
    pub fn new(config: Arc<ClientConfig>) -> Self {
        Self { config }
    }

    /// Issue a request over mTLS. `req.uri()` must use the `https` scheme.
    pub async fn request(
        &self,
        req: Request<Full<Bytes>>,
    ) -> Result<hyper::Response<hyper::body::Incoming>> {
        let uri = req.uri().clone();
        let host = uri.host().unwrap_or("localhost").to_string();
        let port = uri.port_u16().unwrap_or(443);
        let server_name = ServerName::try_from(host.clone())
            .with_context(|| format!("invalid TLS server name: {host}"))?;

        let addr = (host.as_str(), port)
            .to_socket_addrs()
            .context("resolving mTLS peer")?
            .next()
            .context("no address for mTLS peer")?;
        let tcp = TcpStream::connect(addr).await.context("TCP connect to mTLS peer")?;
        let tls = TlsConnector::from(self.config.clone())
            .connect(server_name, tcp)
            .await
            .context("mTLS handshake")?;

        let (mut sender, conn) = hyper::client::conn::http1::Builder::new()
            .handshake(hyper_util::rt::TokioIo::new(tls))
            .await
            .context("HTTP/1 handshake over mTLS")?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                tracing::debug!(error = %e, "mTLS connection closed");
            }
        });

        sender.send_request(req).await.context("mTLS request failed")
    }

    /// Convenience POST with a JSON body.
    pub async fn post_json(
        &self,
        uri: Uri,
        body: Bytes,
    ) -> Result<hyper::Response<hyper::body::Incoming>> {
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Full::new(body))
            .context("building mTLS POST")?;
        self.request(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("scripts")
            .join("mtls-certs")
            .join(name)
    }

    #[test]
    fn builds_server_and_client_configs_from_fixtures() {
        let server = TlsPaths {
            cert: fixture("server.pem"),
            key: fixture("server.key"),
            ca: fixture("ca.pem"),
        };
        let client = TlsPaths {
            cert: fixture("client.pem"),
            key: fixture("client.key"),
            ca: fixture("ca.pem"),
        };
        if !server.cert.exists() || !client.cert.exists() {
            eprintln!("mTLS fixtures missing; run scripts/gen-mtls-certs.sh — skipping");
            return;
        }
        let srv = server_config(&server).expect("server config builds with client-cert auth");
        client_config(&client).expect("client config builds");
    }

    #[test]
    fn from_env_is_none_without_variables() {
        unsafe {
            std::env::remove_var("TPT__GOV__MTLS_CERT");
            std::env::remove_var("TPT__GOV__MTLS_KEY");
            std::env::remove_var("TPT__GOV__MTLS_CA");
        }
        assert!(TlsPaths::from_env().is_none());
    }
}



