# Changelog

All notable changes to `gov-consent` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `verify_grant` and `verify_grants`: fail-closed verification of
  `DataGrantCredential` expiry, Ed25519 signature, and revocation status in one
  call. The batch variant performs a single revocation lookup.
- `RevocationClient`: batch client for the identity server's
  `/v1/grants/status` endpoint, with a configurable request timeout and an
  in-process TTL cache. Constructible directly or from environment variables.
- `issuer_key_from_env` and `decode_public_key`: load the identity server's
  Ed25519 public key from `TPT__GOV__ISSUER_PUBLIC_KEY`, accepting standard or
  URL-safe base64, padded or unpadded.
- Environment configuration: `TPT__GOV__IDENTITY_SERVER_URL`,
  `TPT__GOV__IDENTITY_API_KEY`, `TPT__GOV__REVOCATION_TIMEOUT_SECS`, and
  `TPT__GOV__REVOCATION_CACHE_SECS`.
- Unit tests for public key decoding and for fail-closed behaviour when the
  issuer key is not configured.
