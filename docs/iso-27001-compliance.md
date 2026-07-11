# ISO/IEC 27001 Compliance Documentation

This document maps the **tpt-gov-nz** platform controls to the ISO/IEC 27001:2022
Annex A control themes. It is **not** a certification — engaging a licensed
auditor is tracked separately under *External security audit*. It is the
evidence pack a conformity assessment would review.

## 1. Scope & information security policy (Clauses 4–7)

- **Scope:** The federated platform — citizen portal, staff portal, API gateway,
  identity server, department nodes, ingesters, and federation transport.
- **Information security policy:** `README.md` + `CLAUDE.md` state the
  privacy-by-design and federation-by-consent principles.
- **Roles:** ownership of each department node rests with the deploying agency
  (see *International fork guide* for governance hand-off).

## 2. Annex A control themes (2022)

| Theme | How tpt-gov-nz addresses it | Evidence |
| ----- | --------------------------- | -------- |
| A.5 Organisational controls | Least-privilege, consent-gated data access | `policies/*.rego`, `consent.rs` in each dept node |
| A.6 People controls | Staff auth via RealMe SAML2 (scaffold) + passphrase stand-in | `apps/portal-staff/app/lib/auth-actions.ts` |
| A.7 Technological controls | mTLS between services, Ed25519-signed federation envelopes, JWT sessions in httpOnly cookies | `crates/gov-mtls`, `crates/gov-federation-core`, `portal-citizen/app/lib/session.ts` |
| A.8 Physical controls | Out of scope of software; delegate to hosting provider | Hosting provider SoC 2 / ISO 27001 cert |

### Selected controls detail

- **Cryptographic controls (A.8.24):** Ed25519 for DIDs, credentials and
  federation envelopes (`gov-identity-core`, `gov-federation-core`); TLS 1.3 via
  rustls (`gov-mtls`); AES at rest recommended at the Postgres layer.
- **Logging & monitoring (A.8.15):** every data access writes a signed,
  immutable `AuditLogEntry` (`audit.rs`, `gov-federation-core`); OpenTelemetry
  tracing on the gateway.
- **Access control (A.8.3):** per-department rate limiting, circuit breakers,
  and JWT validation middleware on `gov-gateway`.
- **Data minimisation (A.8.10):** AI `produceAiContext()` emits only
  scope-approved fields; PII redactor strips identifiers before any model call.
- **Zero-knowledge proofs (A.8.9 / privacy):** ZK-SNARK age/residency range
  proofs (`crates/gov-zk`) — demo-grade, unaudited, opt-in.

## 3. Risk treatment

- Identified risks: cross-dept over-collection, insider access, transport
  interception, model data exfiltration.
- Treatments: consent grants enforced by OPA sidecar with local fallback; mTLS
  mesh; sovereign Ollama option (see `docs/sovereign-ollama.md`); automated
  audit trail.

## 4. Supplier & federated-node management

- Each fork/agency operates its own nodes; the reference implementation ships
  with documented deployment (`README.md`, `docker/phase1.yml`,
  `docker/demo.yml`).
- A `CONTRIBUTING.md` and department-addition runbook enforce secure defaults.

## 5. Continual improvement

- CI (` .github/workflows/ci.yml`) runs `clippy`, `fmt`, unit tests, TS
  typecheck/lint on every change.
- Load tests (`load-tests/k6/`) gate capacity changes.

## 6. Gap register (open)

| Gap | Linked TODO item |
| --- | ---------------- |
| Formal external audit not yet performed | External security audit |
| RealMe SAML2 live (currently scaffold) | RealMe SAML2 integration |
| Schema registry for versioned data contracts | Schema registry |
| Māori data sovereignty layer (iwi nodes) | Optional Māori data sovereignty layer |
| ISO 27001 certification audit | External security audit |
