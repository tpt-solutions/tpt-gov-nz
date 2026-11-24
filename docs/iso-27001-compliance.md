# ISO/IEC 27001 Compliance Documentation

This document maps the **tpt-gov-nz** platform controls to the ISO/IEC 27001:2022
Annex A control themes. It is **not** a certification, and it does not claim
conformity. Engaging a licensed auditor is tracked separately under *External
security audit*. It is the evidence pack a conformity assessment would review,
and each control below states what is implemented, what is partial, and what is
planned. Where this document and the code disagree, the code is correct and this
document should be fixed.

Status labels: **Implemented** (in code and tested), **Partial** (in code, not
enforced everywhere), **Planned** (not in code).

## 1. Scope & information security policy (Clauses 4–7)

- **Scope:** The federated platform: citizen portal, staff portal, API gateway,
  identity server, department nodes, ingesters, and federation transport.
- **Information security policy:** `README.md` and `CLAUDE.md` state the
  privacy-by-design and federation-by-consent principles. No formal policy
  document has been approved by an agency.
- **Roles:** ownership of each department node rests with the deploying agency
  (see *International fork guide* for governance hand-off).

## 2. Annex A control themes (2022)

| Theme | Status | How tpt-gov-nz addresses it | Evidence |
| ----- | ------ | --------------------------- | -------- |
| A.5 Organisational controls | Partial | Consent-gated data access in the IRD service and the template, with an OPA policy per department | `policies/*.rego`, `services/gov-dept-ird/src/consent.rs` |
| A.6 People controls | Partial | Staff sign-in through RealMe SAML2 (verified response handling, no live IdP configured) and a passphrase stand-in for development | `apps/portal-staff/app/lib/realme.ts`, `auth-actions.ts` |
| A.7 Technological controls | Partial | Ed25519-signed federation envelopes; JWT sessions in httpOnly cookies; mTLS helpers exist but are not enforced on department endpoints | `crates/gov-federation-core`, `crates/gov-mtls`, `portal-citizen/app/lib/session.ts` |
| A.8 Physical controls | Out of scope | Software project; physical security is delegated to the hosting provider | Hosting provider evidence |

### Selected controls detail

- **Cryptographic controls (A.8.24):** Ed25519 for DIDs, credentials and
  federation envelopes (`gov-identity-core`, `gov-federation-core`). TLS is
  provided by rustls in `gov-mtls`; the protocol-version policy is not pinned in
  code and has not been reviewed. Encryption at rest is not implemented by the
  platform and must be configured at the Postgres or volume layer.
- **Logging & monitoring (A.8.15):** The IRD service writes a read-audit row to
  `data_access_log` for every `/citizen/data` call, granted or denied, before
  data is returned. The table is append-only by trigger. The audit row is not
  signed and is not hash-chained. The reference template (`gov-dept-node`) has a
  hash-chained, signed `AuditLogEntry` path, but it is not yet the path every
  department uses. Rows are not yet exported to a SIEM. OpenTelemetry tracing is
  enabled on the gateway.
- **Access control (A.8.3):** Per-department rate limiting, circuit breakers and
  JWT validation middleware on `gov-gateway`. Department nodes still accept
  requests that carry no federation grant when the caller identifies as the
  citizen or staff. This is the open P0 item tracked in ADR 0002, which decides
  mTLS from the gateway and the staff portal as the fix. Not yet implemented.
- **Federation (A.8.20 / A.8.26):** Federation envelopes are checked for signature,
  addressee, timestamp skew, per-peer rate, replayed message IDs and revoked
  consent grants before dispatch. Node identities persist across restarts when
  `TPT__GOV__FEDERATION_KEY_FILE` or `TPT__GOV__FEDERATION_SECRET_KEY` is set.
  QUIC peers are authenticated by envelope signature only; QUIC mTLS is planned.
- **Data minimisation (A.8.10):** The AI context helpers emit only fields for the
  scopes the citizen granted. The PII redactor removes the identifier types it
  recognises (NHI, IRD number, passport number, phone). It does not remove other
  free-text personal information.
- **Secure development (A.8.25 / A.8.28):** Staff and citizen server actions
  validate scopes, DIDs, action types, parameters and questions before use
  (`packages/@tpt/gov-schema/src/action-input.ts`), and fail closed.
- **Zero-knowledge proofs:** ZK-SNARK age and residency range proofs
  (`crates/gov-zk`) are demonstration code, unaudited, and opt-in.

## 3. Risk treatment

- Identified risks: cross-department over-collection, insider access, transport
  interception, model data exfiltration.
- Treatments: consent grants checked by the local consent module, with an OPA
  sidecar used when the `opa` feature is enabled; a sovereign Ollama option
  (`docs/sovereign-ollama.md`); an audit trail for the IRD service. The mTLS
  mesh in the risk register is planned, not deployed.

## 4. Supplier & federated-node management

- Each fork or agency operates its own nodes. The reference implementation ships
  with deployment documentation (`README.md`, `docker/phase1.yml`,
  `docker/demo.yml`).
- `CONTRIBUTING.md` and the department-addition runbook describe the checklist.
  Secure defaults are not enforced by a generator check yet.

## 5. Continual improvement

- CI (`.github/workflows/ci.yml`) runs `cargo fmt --check`, `cargo clippy` with
  warnings as errors, Rust unit tests, TypeScript typecheck, lint and tests, and
  secret scanning.
- Load-test scripts exist in `load-tests/k6/`. They are run by hand and are not
  run in CI, so they do not gate capacity changes.

## 6. Gap register (open)

| Gap | Linked TODO item |
| --- | ---------------- |
| No formal external audit has been performed | External security audit |
| Department endpoints do not yet authenticate the caller; mTLS from the gateway and the staff portal is decided (ADR 0002) but not implemented | Remove `requesting_dept_id` self-assertion bypass |
| The read audit is implemented in the IRD service only; the other department services have not adopted it | Audit: IRD read audit, real `citizen_did` on actions |
| Audit rows are not signed or hash-chained in the IRD service, and are not exported to a SIEM | Audit; Observability: audit-log export for SIEM |
| RealMe SAML2 has no live IdP configuration, and staff roles are not implemented | RealMe SAML2 integration |
| No code path changes referral status yet; referral status is set to `open` only | Staff portal referrals |
| Schema registry for versioned data contracts (service exists; not enforced by consumers) | Schema registry |
| Māori data sovereignty layer (iwi nodes) | Optional Māori data sovereignty layer |
| ISO 27001 certification audit | External security audit |
