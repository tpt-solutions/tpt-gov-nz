# Changelog

All notable changes to `gov-identity-core` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `GovDid`: `did:gov:nz:<base64url public key>` identifiers, with
  `from_verifying_key` and `parse`.
- `DidDocument`, built from a signing key.
- `VerifiableCredential` with issuer and subject DIDs, credential type, validity
  window, JSON claims, and an Ed25519 proof. Expiry and signature checks are
  provided.
- `DataGrantCredential`: a citizen-issued, time-limited grant authorising a
  requesting department to receive named scopes from a providing department.
  Expiry (`is_valid`) and signature (`verify_signature`) checks are provided.
- `CredentialIssuer`: key generation and restoration from base64, public key
  export, `issue_vc`, and `issue_data_grant`.
- `IdentityError` for identity and verification failures.
