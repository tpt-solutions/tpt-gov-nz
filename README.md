# tpt-gov-nz

An open-source reference implementation of a **unified New Zealand government
services portal**. It demonstrates a citizen-centric model where each government
department owns and serves its own data, and a citizen grants or revokes consent
for any cross-department sharing — with a self-custodied digital identity and an
optional, consent-bounded AI assistant.

> This is a reference/demo project. It is **not** affiliated with any New Zealand
> government agency, and the demo uses entirely fictional data.

## Why

Most government portals centralise a citizen profile. This project explores the
opposite: no central profile, no honeypot. The citizen holds a Decentralised
Identifier (DID) in their own browser; departments serve their own systems; and
every cross-department access requires an explicit, signed, auditable consent
grant.

## Architecture

```
gov-ingester-<id>   pulls from a legacy system (or JSON fixtures) → writes to dept DB
gov-dept-<id>       owns the DB, serves data to the portal + federation, enforces consent
portal-citizen      the citizen-facing Next.js app (this is where you sign in)
portal-staff        read-only case-worker views
gov-identity-server issues DIDs / consent grants (DataGrantCredential)
gov-federation-node routes cross-department requests with signed audit logs
gov-gateway         JWT-validating API gateway / reverse proxy
```

| Layer        | Stack                                   |
|--------------|-----------------------------------------|
| Portals / packages / i18n / UI | TypeScript (Next.js 15, Turbo, pnpm) |
| Services / ingesters / gateway / identity / federation | Rust (Axum, SQLx, Cargo workspace) |

### Identity & consent

- A citizen's wallet (Ed25519, WebCrypto) derives a DID: `did:gov:nz:<pubkey>`.
- On sign-in the portal issues an HS256 session JWT in an httpOnly cookie.
- Cross-department access is gated by an OPA policy (`policies/*.rego`) and a
  signed `DataGrantCredential` issued by the identity server.

### AI

The assistant (`packages/@tpt/gov-ai-client`) is provider-agnostic (OpenRouter,
Ollama). It operates at a configurable level (`none` / `advisory` / `assisted` /
`automated`) and only ever sees consented, PII-redacted context.

## Quick start — demo (one command)

The fastest way to explore the product is the self-contained demo. It runs the
citizen portal on fictional data for **Alex Tane** and contacts no real systems.

```bash
docker compose -f docker/demo.yml up
# open http://localhost:3000
```

Or run locally (no Docker):

```bash
cp .env.example .env          # NEXT_PUBLIC_DEMO_MODE is true by default
pnpm install
pnpm --filter @tpt/portal-citizen dev
```

See [`README-DEMO.md`](./README-DEMO.md) for scenarios, enabling the AI assistant,
and resetting the demo.

## Quick start — full stack (Phase 1)

Requires Docker + Rust + Node/pnpm.

```bash
cp .env.example .env
docker compose -f docker/phase1.yml up -d   # Postgres for 4 depts + identity + audit
# then run the Rust services and the portal (see package scripts / Cargo workspace)
```

## Repository layout

```
apps/
  portal-citizen/   citizen-facing app (auth, dashboard, dept modules, consent, audit, AI)
  portal-staff/      read-only case-worker views
packages/
  @tpt/gov-schema/   shared TypeScript contracts (DID, scopes, grants, AI, dept bundles)
  @tpt/gov-ai-client/ provider-agnostic AI client + PII redactor
  @tpt/gov-i18n/     i18n strings (en-NZ + te reo Māori opt-in)
  adapters/@tpt/adapter-ird/  legacy adapter (being phased out; IRD only)
crates/              gov-identity-core, gov-federation-core, gov-federation-node
services/            gov-gateway, gov-identity-server, gov-dept-*, gov-ingester-*, gov-ai
policies/            OPA (.rego) consent policies, one per department
docker/              phase1.yml (full stack), demo.yml (citizen demo)
```

## Status

- Phase 1 (IRD, WINZ/MSD, MOH, DIA): department services, ingesters, portal
  modules, staff views, federation, and AI are implemented.
- Citizen portal shell + demo version: complete.
- See [`TODO.md`](./TODO.md) for the full, checked task tracker.

## License

[Apache 2.0](./LICENSE).
