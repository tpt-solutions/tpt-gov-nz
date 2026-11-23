//! Test-only helpers for department services.
//!
//! Department services verify grant signatures against the identity server's public key
//! (`TPT__GOV__ISSUER_PUBLIC_KEY`) and check revocation against its grant-status endpoint
//! (`POST /v1/grants/status`). `shared_issuer()` sets both up once per test process: it
//! generates one issuer, points the key variable at it, and starts a mock grant-status server
//! that reports every requested grant as active.
//!
//! This crate is a dev-dependency only (`publish = false`). It must never be linked into a
//! production binary.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::OnceLock;
use std::thread;

use gov_identity_core::CredentialIssuer;

static ISSUER: OnceLock<CredentialIssuer> = OnceLock::new();

/// The shared identity issuer for this test process. The first call starts the mock
/// identity server and sets the environment variables department services read. Every test
/// that issues a grant must call this before it sends a request.
pub fn shared_issuer() -> &'static CredentialIssuer {
    ISSUER.get_or_init(|| {
        let issuer = CredentialIssuer::generate();
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock identity server");
        let addr = listener.local_addr().expect("mock identity server address");
        thread::spawn(move || serve(listener));
        // SAFETY: runs once, inside OnceLock initialisation, before any test reads these variables
        // through a department service. No other code in the test process sets them.
        unsafe {
            std::env::set_var("TPT__GOV__ISSUER_PUBLIC_KEY", issuer.public_key_b64());
            std::env::set_var("TPT__GOV__IDENTITY_SERVER_URL", format!("http://{addr}"));
            std::env::set_var("TPT__GOV__IDENTITY_API_KEY", "test-identity-key");
        }
        issuer
    })
}

fn serve(listener: TcpListener) {
    for stream in listener.incoming().flatten() {
        // One request per connection. A failure affects only that connection.
        let _ = handle(stream);
    }
}

/// Answer one `POST /v1/grants/status`: every id in the request is reported active.
fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?; // request line

    let mut content_length = 0usize;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" {
            break;
        }
        if let Some((name, value)) = line.split_once(':')
            && name.trim().eq_ignore_ascii_case("content-length")
        {
            content_length = value.trim().parse().unwrap_or(0);
        }
    }

    let mut body = vec![0u8; content_length.min(1 << 20)];
    reader.read_exact(&mut body)?;
    let ids: Vec<serde_json::Value> = serde_json::from_slice::<serde_json::Value>(&body)
        .ok()
        .and_then(|v| v.get("ids").and_then(|ids| ids.as_array()).cloned())
        .unwrap_or_default();

    let payload = serde_json::json!({ "active": ids, "revoked": [], "unknown": [] }).to_string();
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        payload.len(),
        payload
    )?;
    stream.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_server_reports_requested_grants_active() {
        let _ = shared_issuer();
        let url = std::env::var("TPT__GOV__IDENTITY_SERVER_URL").unwrap();
        let addr = url.trim_start_matches("http://");
        let id = "3f8e1c2a-0000-4000-8000-000000000001";
        let body = format!("{{\"ids\":[\"{id}\"]}}");

        let mut stream = TcpStream::connect(addr).unwrap();
        write!(
            stream,
            "POST /v1/grants/status HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains(&format!("\"active\":[\"{id}\"]")));
    }

    #[test]
    fn shared_issuer_is_the_same_instance_every_call() {
        let a = shared_issuer() as *const CredentialIssuer;
        let b = shared_issuer() as *const CredentialIssuer;
        assert_eq!(a, b);
    }
}
