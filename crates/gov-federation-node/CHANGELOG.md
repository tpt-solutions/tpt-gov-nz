# Changelog

All notable changes to `gov-federation-node` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `FederationNode` runtime with `new`, `with_handler`, `start`, and `send`, using
  the HTTP mock transport by default.
- HTTP transport: `ENVELOPE_PATH` route at `/federation/v1/envelope`,
  `HttpMockTransport`, `EnvelopeHandler`, and `InboundState`.
- Inbound protections: signature verification, replay detection over a window
  of 10,000 message ids, and a per-peer rate limiter (60 requests per minute,
  up to 100 peers by default).
- Peer configuration from `TPT__GOV__DEPT_ID`, `TPT__GOV__FEDERATION_LISTEN`
  (default `0.0.0.0:7000`), and `TPT__GOV__FEDERATION_PEERS`, in the format
  `dept_id@host:port#base64_public_key`.
- File-backed `NodeRegistry` for peers, enrolment tokens, and last-seen times,
  with token consumption and expiry cleanup.
- Optional `quic` feature: `QuicTransport`, `serve_quic`, `client_endpoint`,
  `server_endpoint`, and ALPN `tpt-gov-nz/1`. On the QUIC inbound path, the
  60-second timestamp skew check and consent grant revocation checks are
  applied.

### Notes

- The HTTP transport is a development mock. It does not provide transport
  encryption or client authentication.
- The `quic` feature is off by default.
