# International Fork Guide

**tpt-gov-nz** is designed to be forked by other governments. This guide covers
governance, renaming, legal, and technical steps to stand up a national instance
of the platform outside Aotearoa New Zealand.

## 1. Philosophy

The platform is **department-agnostic**. New Zealand departments are just the
reference set of native modules. A fork swaps the department list, the language
pack, the identity provider, and the legal/policy layer — the federation and
consent machinery stays intact.

## 2. Pre-fork checklist

- [ ] Confirm open-source licence compatibility. The reference is **Apache-2.0**.
      Keep `LICENSE` and attribution when forking.
- [ ] Choose a project name and country code (used in `TPT__GOV__*` config and
      container prefixes).
- [ ] Identify the departments/agencies to implement (see Phase 1–3 module
      pattern in `TODO.md`). Start with 1–3 high-value modules.
- [ ] Decide the identity provider: a RealMe-style SAML2 IdP, or the bundled
      `gov-identity-server` + browser wallet.

## 3. Renaming & branding

1. Update root `package.json` `name`, `description`.
2. Replace `C:\Programming\tpt-gov-nz\apps\portal-citizen\app\lib\i18n` strings
   (or extend `@tpt/gov-i18n`) with your languages.
3. Swap the logo in `apps/portal-citizen/app/layout.tsx` and the staff portal.
4. Update `README.md` and `README-DEMO.md` quick-start.

## 4. Adding your first department (Stages 1–6)

Follow `CONTRIBUTING.md` → *How to add a new department*. In short:

1. **Stage 1 — Dept service:** `services/gov-dept-<id>` (Rust + sqlx + Axum).
   Copy `services/gov-dept-statsnz` as a template.
2. **Stage 2 — Ingester:** `services/gov-ingester-<id>` pulls from the legacy
   system (start with the `mock` transport + JSON fixtures).
3. **Stage 3 — Portal UI:** `apps/portal-citizen/app/dept/<id>/` (server
   actions + pages). Copy an existing `dept/<id>` directory.
4. **Stage 4 — Staff view:** `apps/portal-staff/app/dept/<id>/` (read-only).
5. **Stage 5 — Federation:** add `policies/<id>.rego` and wire consent in
   `consent.rs`.
6. **Stage 6 — AI:** add `<id>-ai.ts` `produceAiContext()` + an entitlement
   prompt.

Then register the service in root `Cargo.toml` members, add a DB + container to
`docker/phase1.yml`, add env vars to `.env.example`, and a `DataBundle` schema in
`packages/@tpt/gov-schema/src/departments/`.

## 5. Federation across jurisdictions (opt-in)

- Nodes authenticate with Ed25519-signed `FederationEnvelope`s
  (`crates/gov-federation-core`). Cross-border requests still require a
  `DataGrantCredential` — consent travels with the request.
- Use the QUIC transport (`crates/gov-federation-node`, `quic` feature) for
  inter-agency links, pinned with self-signed certs + envelope auth.
- Local council federation is a future opt-in (see `TODO.md`).

## 6. Data sovereignty

- For indigenous/instrumentalized data, stand up iwi- or region-controlled
  nodes behind the same federation (future: *Optional Māori data sovereignty
  layer*). The architecture supports nodes that no central authority can read
  without a grant.

## 7. Operational hand-off

- Replace demo seed data (`apps/portal-citizen` demo fixtures) with real
  ingester transports.
- Move staff auth from the passphrase stand-in to your SAML2 IdP.
- Run the load tests (`load-tests/k6/`) and engage an external auditor before
  go-live.

## 8. Keeping in sync

Track upstream changes via git remote. Because the department pattern is uniform,
most upstream improvements (gateway resilience, mTLS, ZK proofs) apply
unchanged to your fork.
