//! End-to-end checks for `gov_mtls::serve`: a real TLS handshake with client
//! certificates issued by a generated internal CA.

use std::sync::Arc;

use axum::{Extension, Router, routing::get};
use bytes::Bytes;
use gov_mtls::{MtlsClient, PeerCertificate, fingerprint_sha256, serve};
use http_body_util::{BodyExt, Full};
use rcgen::{BasicConstraints, Certificate, CertificateParams, IsCa, KeyPair};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::server::WebPkiClientVerifier;
use rustls::{ClientConfig, RootCertStore, ServerConfig};
use tokio::net::TcpListener;

struct Leaf {
    cert: CertificateDer<'static>,
    key: PrivatePkcs8KeyDer<'static>,
}

fn new_ca() -> (Certificate, KeyPair) {
    gov_mtls::install_crypto_provider();
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(vec![]).unwrap();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    let cert = params.self_signed(&key).unwrap();
    (cert, key)
}

fn issue(ca: &(Certificate, KeyPair), names: Vec<String>) -> Leaf {
    let key = KeyPair::generate().unwrap();
    let cert = CertificateParams::new(names)
        .unwrap()
        .signed_by(&key, &ca.0, &ca.1)
        .unwrap();
    Leaf {
        cert: CertificateDer::from(cert.der().to_vec()),
        key: PrivatePkcs8KeyDer::from(key.serialize_der()),
    }
}

fn roots_for(ca: &(Certificate, KeyPair)) -> RootCertStore {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(ca.0.der().to_vec()))
        .unwrap();
    roots
}

fn server_config(ca: &(Certificate, KeyPair), server: Leaf) -> Arc<ServerConfig> {
    let verifier = WebPkiClientVerifier::builder(Arc::new(roots_for(ca)))
        .build()
        .unwrap();
    Arc::new(
        ServerConfig::builder()
            .with_client_cert_verifier(verifier)
            .with_single_cert(vec![server.cert], PrivateKeyDer::Pkcs8(server.key))
            .unwrap(),
    )
}

fn client(ca: &(Certificate, KeyPair), identity: Option<Leaf>) -> MtlsClient {
    gov_mtls::install_crypto_provider();
    let builder = ClientConfig::builder().with_root_certificates(roots_for(ca));
    let config = match identity {
        Some(leaf) => builder
            .with_client_auth_cert(vec![leaf.cert], PrivateKeyDer::Pkcs8(leaf.key))
            .unwrap(),
        None => builder.with_no_client_auth(),
    };
    MtlsClient::new(Arc::new(config))
}

/// Router that echoes the caller's certificate fingerprint, or "none".
fn echo_router() -> Router {
    Router::new().route(
        "/who",
        get(|peer: Option<Extension<PeerCertificate>>| async move {
            peer.map(|Extension(p)| p.fingerprint_sha256)
                .unwrap_or_else(|| "none".into())
        }),
    )
}

async fn start(config: Arc<ServerConfig>) -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        serve(listener, echo_router(), config).await.unwrap();
    });
    addr
}

async fn get_who(client: &MtlsClient, addr: std::net::SocketAddr) -> anyhow::Result<String> {
    let req = hyper::Request::get(format!("https://127.0.0.1:{}/who", addr.port()))
        .body(Full::new(Bytes::new()))
        .unwrap();
    let resp = client.request(req).await?;
    let body = resp.into_body().collect().await?.to_bytes();
    Ok(String::from_utf8(body.to_vec())?)
}

#[tokio::test]
async fn trusted_client_certificate_reaches_handler_with_its_fingerprint() {
    let ca = new_ca();
    let server = issue(&ca, vec!["localhost".into(), "127.0.0.1".into()]);
    let gateway = issue(&ca, vec!["gateway".into()]);
    let expected = fingerprint_sha256(gateway.cert.as_ref());

    let addr = start(server_config(&ca, server)).await;
    let got = get_who(&client(&ca, Some(gateway)), addr).await.unwrap();
    assert_eq!(got, expected);
}

#[tokio::test]
async fn client_certificate_from_another_ca_is_refused() {
    let ca = new_ca();
    let rogue_ca = new_ca();
    let server = issue(&ca, vec!["localhost".into(), "127.0.0.1".into()]);
    let intruder = issue(&rogue_ca, vec!["intruder".into()]);

    let addr = start(server_config(&ca, server)).await;
    // The handshake fails, so the request never reaches the handler.
    let result = get_who(&client(&ca, Some(intruder)), addr).await;
    assert!(
        result.is_err(),
        "a certificate from another CA must be refused"
    );
}

#[tokio::test]
async fn connection_without_client_certificate_is_refused() {
    let ca = new_ca();
    let server = issue(&ca, vec!["localhost".into(), "127.0.0.1".into()]);

    let addr = start(server_config(&ca, server)).await;
    let result = get_who(&client(&ca, None), addr).await;
    assert!(result.is_err(), "anonymous TLS must be refused");
}

#[tokio::test]
async fn caller_not_on_allow_list_gets_403() {
    let ca = new_ca();
    let server = issue(&ca, vec!["localhost".into(), "127.0.0.1".into()]);
    let gateway = issue(&ca, vec!["gateway".into()]);
    let other = issue(&ca, vec!["other-service".into()]);

    // Only the gateway's certificate is allowed. The other service has a valid
    // CA-issued certificate, so the handshake succeeds and the allow-list refuses it.
    let allowed = std::collections::HashSet::from([fingerprint_sha256(gateway.cert.as_ref())]);
    let app = gov_mtls::restrict_to_peers(echo_router(), allowed);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let config = server_config(&ca, server);
    tokio::spawn(async move {
        serve(listener, app, config).await.unwrap();
    });

    let allowed_resp = get_who(&client(&ca, Some(gateway)), addr).await.unwrap();
    assert_ne!(allowed_resp, "none");

    let denied = hyper::Request::get(format!("https://127.0.0.1:{}/who", addr.port()))
        .body(Full::new(Bytes::new()))
        .unwrap();
    let resp = client(&ca, Some(other)).request(denied).await.unwrap();
    assert_eq!(resp.status(), axum::http::StatusCode::FORBIDDEN);
}

#[test]
fn fingerprint_list_parses_and_rejects_garbage() {
    let fp = "A".repeat(64);
    let parsed = gov_mtls::parse_fingerprint_list(&format!(" {fp} , ,")).unwrap();
    assert_eq!(parsed.len(), 1);
    assert!(parsed.contains(&"a".repeat(64)));
    assert!(gov_mtls::parse_fingerprint_list("not-a-fingerprint").is_err());
    assert!(gov_mtls::parse_fingerprint_list("").unwrap().is_empty());
}
