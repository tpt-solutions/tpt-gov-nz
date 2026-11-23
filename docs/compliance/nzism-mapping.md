# NZISM control mapping (draft)

The New Zealand Information Security Manual (NZISM) is published by the GCSB's
National Cyber Security Centre. This page maps the platform to broad NZISM areas. It
is not a full control-by-control assessment. The mapping must be re-done against the
NZISM version the agency adopts, and it must be reviewed by an assessor.

| NZISM area | Platform position | Status |
|---|---|---|
| Information security governance | `SECURITY.md`, `GOVERNANCE.md`, this pack | Scaffold |
| Information security documentation | ADRs in `docs/adr/`, threat model, PIA template | Scaffold |
| Personnel security | Out of scope for software. Agency responsibility. | Not applicable |
| Physical security | Out of scope for software. Agency responsibility. | Not applicable |
| Information technology security: identification and authentication | Staff passphrase with per-account lockout, RealMe SAML (validated, but fail-open paths remain in TODO), citizen DID-to-key binding | Scaffold |
| Access control and authorisation | Consent-gated cross-department access, OPA policies, staff session verified in middleware and in write actions | Implemented for cross-department access; Scaffold elsewhere |
| Cryptography | Ed25519 for grants and envelopes, HS256 for session tokens, mTLS between services (`crates/gov-mtls`). FIPS-validated modules are not used. | Scaffold |
| Gateways and network security | Strict CSP with nonce, HSTS, frame denial, mTLS for internal services | Scaffold |
| Secure system administration | Docker Compose defaults, `.env.example` required secrets, `make doctor` check | Scaffold |
| Logging and monitoring | Signed, hash-chained audit entries in department nodes. No SIEM export yet. | Scaffold |
| Vulnerability management | Dependabot configuration, CI lint and tests. No `cargo audit` gate yet. | Scaffold |
| Software development | CI for TypeScript and Rust, gitleaks secret scan, OPA check and test in CI | Scaffold |
| Data transfer and disposal | No retention or disposal job yet | Planned |

## Items that need an assessor

- Cryptographic module validation requirements for each classification level
- Hosting and gateway controls for the agency's environment
- Incident response integration with the NCSC reporting process
