# gov-mtls

Mutual TLS configuration and client/server helpers for tpt-gov-nz internal services.

Every internal service (gateway, identity server, department nodes, federation
node) should authenticate its peers with mutual TLS. Each service presents a
certificate signed by a private internal CA and verifies the peer's certificate
against that CA. This crate loads PEM material into `rustls` configurations and
provides an HTTP client that presents the client certificate.

mTLS is opt-in. `TlsPaths::from_env` returns `None` unless all three variables
are set, so local development works without certificates.

## Features

- **PEM loading** for certificates, private keys (PKCS#8), and CA bundles.
- **Server configuration** that requires a valid client certificate issued by
  the internal CA.
- **Client configuration** that presents this service's certificate and verifies
  the server against the CA.
- **`MtlsClient`**, an HTTP client built on `hyper` and `tokio-rustls`, with
  `request` and `post_json` helpers.
- **Environment-driven setup** through `TlsPaths::from_env`.

## Installation

```toml
[dependencies]
gov-mtls = { path = "../../crates/gov-mtls" }
```

## Usage

Build a server configuration that requires client certificates:

```rust,ignore
use gov_mtls::{server_config, TlsPaths};

let paths = TlsPaths::from_env().expect("mTLS is not configured");
let server = server_config(&paths)?;
```

Build a client that calls an internal peer over mTLS:

```rust,ignore
use gov_mtls::{client_config, MtlsClient, TlsPaths};
use std::sync::Arc;

let paths = TlsPaths::from_env().expect("mTLS is not configured");
let client = MtlsClient::new(Arc::new(client_config(&paths)?));
// client.request(...) / client.post_json(...)
```

## Configuration

| Variable | Meaning |
| --- | --- |
| `TPT__GOV__MTLS_CERT` | This service's certificate (PEM). |
| `TPT__GOV__MTLS_KEY` | This service's private key (PEM, PKCS#8). |
| `TPT__GOV__MTLS_CA` | Internal CA certificate(s) (PEM). |

All three must be set. If any is missing, `TlsPaths::from_env` returns `None`.

Generate development certificates with the repository script:

```bash
scripts/gen-mtls-certs.sh
```

## Modules and public API

| Item | Purpose |
| --- | --- |
| `TlsPaths` | Filesystem locations of the certificate, key, and CA. `from_env` reads them. |
| `server_config` | `rustls::ServerConfig` that requires a client certificate. |
| `client_config` | `rustls::ClientConfig` that presents a client certificate. |
| `MtlsClient` | HTTP client that issues requests over mTLS. `new`, `request`, `post_json`. |

## Security notes

- **Client certificates are required on the server side.** `server_config`
  rejects connections that do not present a certificate signed by the CA.
- **Private keys stay on disk with restricted permissions.** Keep `TPT__GOV__MTLS_KEY`
  readable only by the service account, and never commit it.
- **Use separate certificates per service.** A shared identity makes it
  impossible to tell which service made a request.
- **The development CA is not for production.** Use an organisation-managed CA
  and rotate certificates on a schedule.

## Testing

```bash
cargo test -p gov-mtls
```

## Related crates

- [`gov-federation-node`](../gov-federation-node/README.md) can run its HTTP
  transport behind mTLS.

## Contributing

See [CHANGELOG.md](CHANGELOG.md) for release history. Contributions follow the
repository guidelines in the root [README](../../README.md).

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE),
at your option.
