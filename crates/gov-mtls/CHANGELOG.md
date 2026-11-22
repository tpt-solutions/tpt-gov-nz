# Changelog

All notable changes to `gov-mtls` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `TlsPaths`, with `from_env` reading `TPT__GOV__MTLS_CERT`,
  `TPT__GOV__MTLS_KEY`, and `TPT__GOV__MTLS_CA`. Returns `None` unless all three
  are set, so mTLS is opt-in.
- `server_config`: a `rustls` server configuration that requires a client
  certificate signed by the internal CA.
- `client_config`: a `rustls` client configuration that presents this service's
  certificate and verifies the server against the internal CA.
- `MtlsClient`: an HTTP client over `hyper` and `tokio-rustls`, with `request`
  and `post_json` helpers.
- PEM parsing for certificates, PKCS#8 private keys, and CA bundles.
