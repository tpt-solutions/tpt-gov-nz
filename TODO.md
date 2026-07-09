# tpt-gov-nz — Task Tracker

> Legend: ✅ Done · 🔄 In progress · ⬜ Todo · 🎯 Demo-critical · ~~Removed~~

**Architecture:**
- **TypeScript** (Turbo + pnpm) — portals, packages, i18n, UI
- **Rust** (Cargo workspace) — API gateway, identity server, dept services, ingesters, federation

**Department pattern — straight to native, no adapters:**
```
gov-ingester-<id>   pulls from legacy system → writes to dept DB
gov-dept-<id>       owns the DB, serves data to portal + federation
portal-citizen/app/dept/<id>/   calls gov-dept-<id> directly
```

---

## Phase 0 — Scaffold & Architecture ✅

- [x] Monorepo root config (Turbo + pnpm + Cargo workspace)
- [x] `tsconfig.base.json`
- [x] `.gitignore`
- [x] `CLAUDE.md`
- [x] `TODO.md`

---

## Phase 1 — Core Infrastructure

### Schema & Contracts (`packages/@tpt/gov-schema`) — TypeScript
- [x] `identity.ts` — DID, scopes, consent grants, `CitizenIdentityToken`
- [x] `federation.ts` — `FederationEnvelope`, `AuditLogEntry`
- [x] `ai.ts` — AI levels, providers, `AiAction`, `AiContextChunk`
- [x] `departments/adapter.ts` — `DeptAdapter` as contract/docs type only (not instantiated at runtime)
- [x] `departments/ird.ts` — full schemas: `IRDDataBundle`, `IRDTaxAssessment`, `IRDGstPeriod`, `IRDKiwiSaver`, `IRDWorkingForFamilies`, `IRDAction`
- [x] `departments/winz.ts` — `WINZDataBundle`
- [x] `departments/moh.ts` — `MOHDataBundle`
- [x] `departments/dia.ts` — `DIADataBundle`
- [ ] Unit tests for all schema files

### ~~Adapter packages~~ — Removed
> Dropped in favour of portal calling dept services directly. Ingesters handle legacy integration.
- ~~`packages/adapters/@tpt/adapter-ird`~~
- ~~`packages/adapters/@tpt/adapter-winz`~~
- ~~`packages/adapters/@tpt/adapter-moh`~~
- ~~`packages/adapters/@tpt/adapter-dia`~~

### AI Client (`packages/@tpt/gov-ai-client`) — TypeScript
- [x] Provider-agnostic client (`GovAiClient`)
- [x] OpenRouter provider
- [x] Ollama provider
- [x] PII redactor (NHI, IRD number, passport, phone)
- [ ] Unit tests for PII redactor
- [ ] Unit tests for `GovAiClient` (mock providers)

### Rust — Identity Core (`crates/gov-identity-core`)
- [x] `GovDid` — DID generation + parsing
- [x] `DidDocument` — public key + metadata only
- [x] `VerifiableCredential` — expiry checking
- [x] `DataGrantCredential` — consent grant VC
- [ ] Ed25519 signature verification on credential proof
- [ ] Credential issuance (sign with identity server keypair)
- [ ] Unit tests

### Rust — Federation Core (`crates/gov-federation-core`)
- [x] `FederationEnvelope` — message type, routing, consent grant IDs
- [x] `NodeKeypair` — Ed25519 key generation
- [x] `AuditLogEntry` — signed immutable log
- [x] `FederationError`
- [ ] Envelope signing (Ed25519)
- [ ] Envelope signature verification
- [ ] Unit tests for sign + verify round-trip

### Rust — Federation Node (`crates/gov-federation-node`)
- [x] `FederationNodeConfig` + `FederationNode` stub
- [ ] HTTP mock transport for Phase 1 local dev
- [ ] QUIC transport (Phase 2)

### Rust — API Gateway (`services/gov-gateway`)
- [x] Axum skeleton + `/health` + `/v1/citizen/resolve` stub
- [ ] JWT validation middleware
- [ ] Per-department rate limiting
- [ ] Route `/v1/dept/:dept_id/*` → dept service proxy
- [ ] Circuit breaker middleware
- [ ] OpenTelemetry tracing

### Rust — Identity Server (`services/gov-identity-server`)
- [x] Axum skeleton + route stubs
- [ ] Database migrations
- [ ] `POST /v1/did/register`
- [ ] `GET /v1/did/:did`
- [ ] `POST /v1/grants` — issue `DataGrantCredential`
- [ ] `DELETE /v1/grants/:id` — revoke consent
- [ ] `GET /v1/grants?citizen_did=`

### Rust — Dept Node Template (`services/gov-dept-node`)
- [x] Axum skeleton + route stubs
- [ ] OPA sidecar integration (consent verification)
- [ ] Audit log write on every data access

### Docker / Infrastructure
- [x] `docker/phase1.yml` — 4 dept Postgres nodes + identity DB + audit DB
- [x] `gov-dept-ird` container + healthcheck in `phase1.yml`
- [ ] Add gov-gateway container to `phase1.yml`
- [ ] Add gov-identity-server container to `phase1.yml`
- [ ] `.env.example` — all required env vars

---

## IRD — Full Native Module 🔄

### Stage 1 — Dept Service (`services/gov-dept-ird`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`
- [x] `src/main.rs` — Axum server, sqlx pool, run migrations on startup
- [x] `src/error.rs` — `IrdError` with `IntoResponse`
- [x] `src/db.rs` — all sqlx queries (resolve, tax, income, GST, KiwiSaver, WFF, actions log)
- [x] `src/routes.rs` — `/health`, `/citizen/resolve`, `/citizen/data`, `/citizen/action`, `/citizen/:did/tax-years`, `/citizen/:did/gst-periods`
- [x] `src/actions.rs` — `update-kiwisaver-rate`, `file-gst-return`, `request-tax-summary`
- [x] Migrations 001–008 (citizens, income, tax assessments, GST, KiwiSaver, WFF, actions log, dev seed)
- [ ] Unit tests: resolve (found + not found)
- [ ] Unit tests: fetch_data with each scope combination
- [ ] Unit tests: actions (valid + invalid inputs)
- [ ] Integration test: full HTTP round-trip with real PgPool

### Stage 2 — Ingester (`services/gov-ingester-ird`) — Rust ⬜
- [ ] `Cargo.toml`, basic structure
- [ ] Ingester trait/interface (swap transport without changing logic)
- [ ] Mock transport — reads from JSON fixture files (for dev/demo, no legacy system needed)
- [ ] SFTP transport stub — structure for real IRD batch file integration
- [ ] Transform layer — maps raw IRD data format → dept DB schema
- [ ] Scheduler — run on configurable interval (cron or event-triggered)
- [ ] Idempotent upserts (safe to re-run)
- [ ] Ingestion audit log (what was pulled, when, row counts)

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/ird/`) — TypeScript ✅🔄
- [x] `actions.ts` — `fetchIrdData`, `submitIrdAction` (server actions calling `gov-dept-ird`)
- [x] `page.tsx` — IRD overview (income, refund, WFF summary, KiwiSaver summary)
- [x] `tax-summary/page.tsx` — all tax years, income breakdown
- [ ] `working-for-families/page.tsx` — eligibility, entitlement breakdown, payment frequency
- [ ] `kiwisaver/page.tsx` — status, rate, balance, first-home eligibility
- [ ] `kiwisaver/update-rate/page.tsx` — rate change form (AI-assistable at level ≥ assisted)
- [ ] `gst/page.tsx` — registration status, period history
- [ ] `gst/file-return/page.tsx` — GST filing form
- [ ] Error states + loading skeletons on all pages

### Stage 4 — Staff View (`apps/portal-staff/app/dept/ird/`) — TypeScript ⬜
- [ ] Read-only case worker view (same data, no action buttons)

### Stage 5 — Federation ⬜
- [ ] OPA policy file (`policies/ird.rego`)
- [ ] Consent verification wired into `/citizen/data`
- [ ] Cross-dept data request test (e.g. WINZ requests IRD income)

### Stage 6 — AI Integration ⬜
- [ ] Richer `produceAiContext()` for WFF + KiwiSaver
- [ ] Life-event wizard: "I just had a baby" → WFF eligibility check
- [ ] Entitlement prompt: "Am I eligible for WFF?"
- [ ] AI action suggestion (level ≥ assisted): KiwiSaver rate recommendation

---

## WINZ — Full Native Module ⬜

### Stage 1 — Dept Service (`services/gov-dept-winz`) — Rust
- [ ] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions)
- [ ] Migrations: citizens, benefits, payments, case_notes, actions_log
- [ ] Dev seed: test citizen with jobseeker + accommodation supplement

### Stage 2 — Ingester (`services/gov-ingester-winz`) — Rust
- [ ] Mock transport (JSON fixtures)
- [ ] WINZ legacy system transport stub
- [ ] Transform + idempotent upsert

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/winz/`) — TypeScript
- [ ] `actions.ts`
- [ ] Overview page — active benefits, weekly total, next appointment
- [ ] Benefits detail page
- [ ] Payment history page

### Stage 4 — Staff View + Stage 5 — Federation + Stage 6 — AI
- [ ] Staff view, OPA policy, AI benefit calculator

---

## MOH — Full Native Module ⬜

### Stage 1 — Dept Service (`services/gov-dept-moh`) — Rust
- [ ] `Cargo.toml`, `Dockerfile`, `src/`
- [ ] Migrations: citizens (NHI), gp_enrolments, prescriptions, appointments, vaccinations, actions_log
- [ ] Dev seed: test citizen with GP, prescription, appointment

### Stage 2 — Ingester (`services/gov-ingester-moh`) — Rust
- [ ] Mock transport + NHI/HPI system transport stub
- [ ] Transform + upsert

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/moh/`) — TypeScript
- [ ] `actions.ts`
- [ ] Overview page — NHI, GP, prescriptions, appointments
- [ ] Prescriptions page (repeat request action)

### Stage 4 — Staff View + Stage 5 — Federation + Stage 6 — AI
- [ ] Staff view, OPA policy, AI health navigation

---

## DIA — Full Native Module ⬜

### Stage 1 — Dept Service (`services/gov-dept-dia`) — Rust
- [ ] `Cargo.toml`, `Dockerfile`, `src/`
- [ ] Migrations: citizens, passports, birth_certs, citizenship_records, actions_log
- [ ] Dev seed: test citizen with passport expiring 2028

### Stage 2 — Ingester (`services/gov-ingester-dia`) — Rust
- [ ] Mock transport + DIA system transport stub

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/dia/`) — TypeScript
- [ ] `actions.ts`
- [ ] Overview page — passport, citizenship, documents
- [ ] Passport renewal page
- [ ] Birth certificate request page

### Stage 4 — Staff View + Stage 5 — Federation + Stage 6 — AI
- [ ] Staff view, OPA policy, AI: "my passport is expiring, what do I do?"

---

## Phase 1 — Citizen Portal Shell — TypeScript

- [x] `apps/portal-citizen` — Next.js 15 scaffold + security headers
- [x] Home page (`/`) + Dashboard skeleton (`/dashboard`)
- [ ] Global layout — header, footer, skip-to-content
- [ ] Language toggle (English default / te reo Māori opt-in)
- [ ] Error boundary + 404 + 500 pages
- [ ] PWA manifest + service worker
- [ ] `/login` — Sign in page
- [ ] Browser VC wallet (WebCrypto Ed25519, `localStorage`)
- [ ] DID generation on first login → register with `gov-identity-server`
- [ ] Session management (JWT in httpOnly cookie)
- [ ] `/logout`
- [ ] Dashboard — dept module cards, grant/revoke consent flow
- [ ] Audit trail page (`/audit`)
- [ ] AI chat widget (hidden when `level=none`)

---

## Phase 1 — Demo Version 🎯

- [ ] Demo mode flag: `NEXT_PUBLIC_DEMO_MODE=true`
- [ ] Mock data layer — JSON fixtures replacing live service calls
- [ ] Fictional citizen "Alex Tane" seeded across all 4 depts
- [ ] 3 selectable scenarios (standard, beneficiary, new parent)
- [ ] "Reset demo" button
- [ ] Demo banner (persistent)
- [ ] Guided tour — 5-step walkthrough
- [ ] AI pre-enabled in `advisory` mode
- [ ] `docker/demo.yml` — one-command self-hosted demo
- [ ] `README-DEMO.md` — "clone and run in 2 commands"

---

## Phase 2 — Six More Departments (each Stages 1–6)

Each: dept service → ingester → portal pages → staff view → federation → AI.

- [ ] **NZTA** — driver licence, vehicles, RUC
- [ ] **ACC** — claims, entitlements, rehabilitation
- [ ] **MoE / NZQA** — qualifications, transcripts, student loans
- [ ] **MSD** — work and employment history
- [ ] **MBIE** — business registrations, company director lookup
- [ ] **LINZ** — property titles, land ownership

### Staff Portal (`apps/portal-staff`) — TypeScript
- [ ] Next.js 15 scaffold
- [ ] Staff authentication
- [ ] Citizen search (consent-gated)
- [ ] Cross-dept case view, case notes, referral flow

### Production Hardening
- [ ] RealMe SAML2 integration
- [ ] mTLS between all internal services
- [ ] QUIC transport in `gov-federation-node`
- [ ] ZK-SNARK age/residency proofs
- [ ] Schema registry
- [ ] External security audit + load test (k6)
- [ ] te reo Māori opt-in toggle

---

## Phase 3 — Full Vision

- [ ] All remaining NZ departments (each Stages 1–6)
- [ ] `apps/portal-policy` — AI policy simulation
- [ ] AI level 3 (automated) — routine benefit renewals, document classification
- [ ] Sovereign Ollama deployment guide (government hardware)
- [ ] Local council federation (opt-in)
- [ ] ISO 27001 compliance documentation
- [ ] International fork guide
- [ ] Optional Māori data sovereignty layer (iwi-controlled nodes)

---

## Ongoing / Cross-cutting

- [ ] `packages/@tpt/gov-ui` — shared design system (TypeScript / React)
- [ ] `packages/@tpt/gov-i18n` — i18n strings (en-NZ + te reo Māori opt-in)
- [ ] CI pipeline (GitHub Actions) — lint, typecheck, `turbo test`, `cargo test --workspace`
- [ ] `LICENSE` — Apache 2.0
- [ ] `README.md` — overview, quick start, architecture diagram
- [ ] `CONTRIBUTING.md` — how to add a new department (Stages 1–6)
