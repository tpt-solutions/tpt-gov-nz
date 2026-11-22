# Changelog

All notable changes to `gov-federation-core` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `FederationEnvelope` with canonical-JSON Ed25519 signing (`sign`) and
  verification (`verify`), and a `new_request` constructor for `DATA_REQUEST`
  messages. Wire fields are camelCase.
- `FederationMessageType`: `DATA_REQUEST`, `DATA_RESPONSE`, `DATA_DENIED`, and
  `AUDIT_ACK`.
- `AuditLogEntry` with signing and verification, for the signed audit trail
  that both sides of an exchange write.
- `NodeKeypair` for generating and restoring a department's Ed25519 keypair,
  including `from_secret_b64_with_dept_id`, and `DeptIdentity`.
- `FederationError` covering invalid consent, signature failure, unauthorised
  scope, expired grant, unknown department, serialisation, crypto, and transport
  errors.
