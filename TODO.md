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
- [x] Unit tests for all schema files

### ~~Adapter packages~~ — Removed
> Departments call dept services directly and build AI context inline (`app/dept/<id>/<id>-ai.ts` / `ai-context.ts`); the `DeptAdapter` pattern is no longer used.
- ~~`packages/adapters/@tpt/adapter-ird`~~
- ~~`packages/adapters/@tpt/adapter-winz`~~
- ~~`packages/adapters/@tpt/adapter-moh`~~
- ~~`packages/adapters/@tpt/adapter-dia`~~

### AI Client (`packages/@tpt/gov-ai-client`) — TypeScript
- [x] Provider-agnostic client (`GovAiClient`)
- [x] OpenRouter provider
- [x] Ollama provider
- [x] PII redactor (NHI, IRD number, passport, phone)
- [x] Unit tests for PII redactor
- [x] Unit tests for `GovAiClient` (mock providers)

### Rust — Identity Core (`crates/gov-identity-core`)
- [x] `GovDid` — DID generation + parsing
- [x] `DidDocument` — public key + metadata only
- [x] `VerifiableCredential` — expiry checking
- [x] `DataGrantCredential` — consent grant VC
- [x] Ed25519 signature verification on credential proof
- [x] Credential issuance (sign with identity server keypair)
- [x] Unit tests

### Rust — Federation Core (`crates/gov-federation-core`)
- [x] `FederationEnvelope` — message type, routing, consent grant IDs
- [x] `NodeKeypair` — Ed25519 key generation
- [x] `AuditLogEntry` — signed immutable log
- [x] `FederationError`
- [x] Envelope signing (Ed25519)
- [x] Envelope signature verification
- [x] Unit tests for sign + verify round-trip

### Rust — Federation Node (`crates/gov-federation-node`)
- [x] `FederationNodeConfig` + `FederationNode` stub
- [x] HTTP mock transport for Phase 1 local dev
- [x] QUIC transport (Phase 2) — `quic.rs`, gated behind `quic` feature, pinned self-signed cert + Ed25519 envelope auth

### Rust — API Gateway (`services/gov-gateway`)
- [x] Axum skeleton + `/health` + `/v1/citizen/resolve` stub
- [x] JWT validation middleware
- [x] Per-department rate limiting
- [x] Route `/v1/dept/:dept_id/*` → dept service proxy
- [x] Circuit breaker middleware
- [x] OpenTelemetry tracing

### Rust — Identity Server (`services/gov-identity-server`)
- [x] Axum skeleton + route stubs
- [x] Database migrations
- [x] `POST /v1/did/register`
- [x] `GET /v1/did/:did`
- [x] `POST /v1/grants` — issue `DataGrantCredential`
- [x] `DELETE /v1/grants/:id` — revoke consent
- [x] `GET /v1/grants?citizen_did=`

### Rust — Dept Node Template (`services/gov-dept-node`)
- [x] Axum skeleton + route stubs
- [x] OPA sidecar integration (consent verification, with local fallback in `consent.rs`)
- [x] Audit log write on every data access (`audit.rs`, table ensured on startup)

### Docker / Infrastructure
- [x] `docker/phase1.yml` — 4 dept Postgres nodes + identity DB + audit DB
- [x] `gov-dept-ird` container + healthcheck in `phase1.yml`
- [x] Add gov-gateway container to `phase1.yml`
- [x] Add gov-identity-server container to `phase1.yml`
- [x] `.env.example` — all required env vars

---

## IRD — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-ird`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`
- [x] `src/main.rs` — Axum server, sqlx pool, run migrations on startup
- [x] `src/error.rs` — `IrdError` with `IntoResponse`
- [x] `src/db.rs` — all sqlx queries (resolve, tax, income, GST, KiwiSaver, WFF, actions log)
- [x] `src/routes.rs` — `/health`, `/citizen/resolve`, `/citizen/data`, `/citizen/action`, `/citizen/:did/tax-years`, `/citizen/:did/gst-periods`
- [x] `src/actions.rs` — `update-kiwisaver-rate`, `file-gst-return`, `request-tax-summary`
- [x] Migrations 001–008 (citizens, income, tax assessments, GST, KiwiSaver, WFF, actions log, dev seed)
- [x] Unit tests: resolve (found + not found)
- [x] Unit tests: fetch_data with each scope combination
- [x] Unit tests: actions (valid + invalid inputs)
- [x] Integration test: full HTTP round-trip with real PgPool

### Stage 2 — Ingester (`services/gov-ingester-ird`) — Rust ✅
- [x] `Cargo.toml`, basic structure
- [x] Ingester trait/interface (swap transport without changing logic)
- [x] Mock transport — reads from JSON fixture files (for dev/demo, no legacy system needed)
- [x] SFTP transport stub — structure for real IRD batch file integration
- [x] Transform layer — maps raw IRD data format → dept DB schema
- [x] Scheduler — run on configurable interval (cron or event-triggered)
- [x] Idempotent upserts (safe to re-run)
- [x] Ingestion audit log (what was pulled, when, row counts)

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/ird/`) — TypeScript ✅
- [x] `actions.ts` — `fetchIrdData`, `submitIrdAction` (server actions calling `gov-dept-ird`)
- [x] `page.tsx` — IRD overview (income, refund, WFF summary, KiwiSaver summary)
- [x] `tax-summary/page.tsx` — all tax years, income breakdown
- [x] `working-for-families/page.tsx` — eligibility, entitlement breakdown, payment frequency
- [x] `kiwisaver/page.tsx` — status, rate, balance, first-home eligibility
- [x] `kiwisaver/update-rate/page.tsx` — rate change form (AI-assistable at level ≥ assisted)
- [x] `gst/page.tsx` — registration status, period history
- [x] `gst/file-return/page.tsx` — GST filing form
- [x] Error states + loading skeletons on all pages

### Stage 4 — Staff View (`apps/portal-staff/app/dept/ird/`) — TypeScript ✅
- [x] Read-only case worker view (same data, no action buttons)

### Stage 5 — Federation ✅
- [x] OPA policy file (`policies/ird.rego`)
- [x] Consent verification wired into `/citizen/data`
- [x] Cross-dept data request test (e.g. WINZ requests IRD income)

### Stage 6 — AI Integration ✅
- [x] Richer `produceAiContext()` for WFF + KiwiSaver
- [x] Life-event wizard: "I just had a baby" → WFF eligibility check
- [x] Entitlement prompt: "Am I eligible for WFF?"
- [x] AI action suggestion (level ≥ assisted): KiwiSaver rate recommendation

---

## WINZ — Full Native Module ✅

> Note: WINZ (Work and Income) is a service line of the **Ministry of Social Development (MSD)**, not a separate department. This module covers Work and Income specifically; broader MSD services (e.g. StudyLink) are scoped under the Phase 2 `MSD` line.

### Stage 1 — Dept Service (`services/gov-dept-winz`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions)
- [x] Migrations: citizens, benefits, payments, case_notes, actions_log, ingestion_runs, payments idempotency key
- [x] Dev seed: test citizen with jobseeker + accommodation supplement
- [x] Unit + integration tests (resolve, fetch_data, actions, consent)
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-winz` + `gov-ingester-winz` containers in `docker/phase1.yml`, DB healthcheck

### Stage 2 — Ingester (`services/gov-ingester-winz`) — Rust ✅
- [x] Mock transport (JSON fixtures)
- [x] WINZ legacy system transport stub
- [x] Transform + idempotent upsert
- [x] Scheduler (configurable interval)

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/winz/`) — TypeScript ✅
- [x] `actions.ts`
- [x] Overview page — active benefits, weekly total, next appointment
- [x] Benefits detail page
- [x] Payment history page
- [x] Benefit review + request-appointment action pages

### Stage 4 — Staff View (`apps/portal-staff/app/dept/winz/`) — TypeScript ✅
- [x] Read-only case worker view (same data, no action buttons)

### Stage 5 — Federation ✅
- [x] OPA policy file (`policies/winz.rego`)
- [x] Consent verification wired into `/citizen/data` (`consent.rs` + `opa.rs` fallback)
- [x] Cross-dept data request test (IRD requests WINZ benefits, with + without grant)

### Stage 6 — AI ✅
- [x] Entitlement prompt: "What support am I entitled to?" (`ai-prompt.tsx`)

---

## MOH — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-moh`) — Rust
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions, consent, opa)
- [x] Migrations: citizens (NHI), gp_enrolments, prescriptions, appointments, vaccinations, actions_log, ingestion_runs, ingester idempotency
- [x] Dev seed: test citizen with GP, prescription, appointment, vaccinations

### Stage 2 — Ingester (`services/gov-ingester-moh`) — Rust
- [x] Mock transport (JSON fixtures)
- [x] NHI/HPI system transport stub
- [x] Transform + idempotent upsert

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/moh/`) — TypeScript
- [x] `actions.ts`
- [x] Overview page — NHI, GP, prescriptions, appointments, vaccinations
- [x] Prescriptions page (repeat request action)

### Stage 4 — Staff View + Stage 5 — Federation + Stage 6 — AI
- [x] Staff view (overview / prescriptions / appointments / vaccinations), OPA policy (`policies/moh.rego`), AI health navigation

---

## DIA — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-dia`) — Rust
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions, consent, opa)
- [x] Migrations: citizens, passports, birth_certs, citizenship_records, actions_log, ingestion_runs, ingester idempotency
- [x] Dev seed: test citizen with passport expiring 2028

### Stage 2 — Ingester (`services/gov-ingester-dia`) — Rust
- [x] Mock transport (JSON fixtures)
- [x] DIA system transport stub

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/dia/`) — TypeScript
- [x] `actions.ts`
- [x] Overview page — passport, citizenship, documents
- [x] Passport renewal page
- [x] Birth certificate request page

### Stage 4 — Staff View + Stage 5 — Federation + Stage 6 — AI
- [x] Staff view (overview / passports / documents), OPA policy (`policies/dia.rego`), AI passport-expiry help

---

## Phase 1 — Citizen Portal Shell — TypeScript

- [x] `apps/portal-citizen` — Next.js 15 scaffold + security headers
- [x] Home page (`/`) + Dashboard skeleton (`/dashboard`)
- [x] Global layout — header, footer, skip-to-content
- [x] Language toggle (English default / te reo Māori opt-in)
- [x] Error boundary + 404 + 500 pages
- [x] PWA manifest + service worker
- [x] `/login` — Sign in page
- [x] Browser VC wallet (WebCrypto Ed25519, `localStorage`)
- [x] DID generation on first login → register with `gov-identity-server`
- [x] Session management (JWT in httpOnly cookie)
- [x] `/logout`
- [x] Dashboard — dept module cards, grant/revoke consent flow
- [x] Audit trail page (`/audit`)
- [x] AI chat widget (hidden when `level=none`)

---

## Phase 1 — Demo Version 🎯

- [x] Demo mode flag: `NEXT_PUBLIC_DEMO_MODE=true`
- [x] Mock data layer — JSON fixtures replacing live service calls
- [x] Fictional citizen "Alex Tane" seeded across all 4 depts
- [x] 3 selectable scenarios (standard, beneficiary, new parent)
- [x] "Reset demo" button
- [x] Demo banner (persistent)
- [x] Guided tour — 5-step walkthrough
- [x] AI pre-enabled in `advisory` mode
- [x] `docker/demo.yml` — one-command self-hosted demo
- [x] `README-DEMO.md` — "clone and run in 2 commands"

---

## Phase 2 — Six More Departments (each Stages 1–6)

Each: dept service → ingester → portal pages → staff view → federation → AI.

- [x] **NZTA** — driver licence, vehicles, RUC
- [x] **ACC** — claims, entitlements, rehabilitation
 - [x] **MoE / NZQA** — qualifications, transcripts
 - [x] **MSD** — student loans/allowances (StudyLink), broader case history beyond Work and Income (see "WINZ — Full Native Module" above)
 - [x] **MBIE** — business registrations, company director lookup
 - [x] **LINZ** — property titles, land ownership

## NZTA — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-nzta`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions, consent, opa)
- [x] Migrations: citizens, driver_licences, vehicles, ruc_records, actions_log, ingestion_runs, ingester idempotency
- [x] Dev seed: test citizens "Alex Tane" + "Bree Kāre"
- [x] Unit + integration tests (resolve, fetch_data, actions, consent)
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-nzta` + `gov-ingester-nzta` containers in `docker/phase1.yml`, DB healthcheck

### Stage 2 — Ingester (`services/gov-ingester-nzta`) — Rust ✅
- [x] Mock transport (JSON fixtures)
- [x] NZTA legacy system transport stub
- [x] Transform + idempotent upsert
- [x] Scheduler (configurable interval)

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/nzta/`) — TypeScript ✅
- [x] `actions.ts`
- [x] Overview page — driver licence, vehicles, RUC
- [x] Renew vehicle registration page (action form)
- [x] Request licence replacement page (action form)

### Stage 4 — Staff View (`apps/portal-staff/app/dept/nzta/`) — TypeScript ✅
- [x] Read-only case worker view (same data, no action buttons)

### Stage 5 — Federation ✅
- [x] OPA policy file (`policies/nzta.rego`)
- [x] Consent verification wired into `/citizen/data` (`consent.rs` + `opa.rs` fallback)
- [x] Cross-dept data request test (IRD requests NZTA licence, with + without grant)

### Stage 6 — AI ✅
- [x] AI context: licence/RUC expiry + vehicle navigation (`nzta-ai.ts`)
- [x] Entitlement prompt: "Is my licence about to expire?" (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `NZTADataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry + demo mock data (3 scenarios)
- [x] `.env.example` NZTA service + gateway + ingester env vars

## ACC — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-acc`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions, consent, opa)
- [x] Migrations: citizens, acc_claims, acc_entitlements, acc_rehabilitation, actions_log, ingestion_runs, ingester idempotency
- [x] Dev seed: test citizens "Alex Tane" + "Bree Kare" with open claim, entitlement, rehab plan
- [x] Unit + integration tests (resolve, fetch_data, actions, consent)
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-acc` + `gov-ingester-acc` containers in `docker/phase1.yml`, DB healthcheck

### Stage 2 — Ingester (`services/gov-ingester-acc`) — Rust ✅
- [x] Mock transport (JSON fixtures)
- [x] ACC legacy system transport stub
- [x] Transform + idempotent upsert
- [x] Scheduler (configurable interval)

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/acc/`) — TypeScript ✅
- [x] `actions.ts`
- [x] Overview page — claims, entitlements, rehabilitation
- [x] Claims page + lodge-claim form
- [x] Entitlements page
- [x] Rehabilitation page
- [x] `loading.tsx`

### Stage 4 — Staff View (`apps/portal-staff/app/dept/acc/`) — TypeScript ✅
- [x] Read-only case worker view (overview + claims/entitlements/rehabilitation subpages)

### Stage 5 — Federation ✅
- [x] OPA policy file (`policies/acc.rego`)
- [x] Consent verification wired into `/citizen/data` (`consent.rs` + `opa.rs` fallback)
- [x] Cross-dept data request test (IRD requests ACC claim, with + without grant)

### Stage 6 — AI ✅
- [x] AI context: claims/entitlements/rehabilitation (`acc-ai.ts`)
- [x] Entitlement prompt: "What is the status of my ACC claim?" (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `ACCDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry + demo mock data (3 scenarios)
- [x] `.env.example` ACC service + gateway + ingester env vars

## MOJ — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-moj`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions, consent, opa)
- [x] Migrations: citizens, moj_fines, moj_disputes, moj_court_records, actions_log, ingestion_runs, ingester idempotency
- [x] Dev seed + unit/integration tests (`tests.rs`)
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-moj` + `gov-ingester-moj` containers in `docker/phase1.yml`, DB healthcheck

### Stage 2 — Ingester (`services/gov-ingester-moj`) — Rust ✅
- [x] Mock transport (JSON fixtures) + legacy transport stub + idempotent upsert

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/moj/`) — TypeScript ✅
- [x] `actions.ts`, overview page, fines (+ pay form), disputes (+ lodge form), court-records, name-change (+ form), `loading.tsx`

### Stage 4 — Staff View (`apps/portal-staff/app/dept/moj/`) — TypeScript ✅
- [x] Read-only case worker view (overview / fines / disputes / court-records)

### Stage 5 — Federation ✅
- [x] OPA policy file (`policies/moj.rego`), consent verification wired into `/citizen/data`

### Stage 6 — AI ✅
- [x] AI context (`moj-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `MOJDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` MOJ service + gateway + ingester env vars

---

## Police — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-police`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions, consent, opa)
- [x] Migrations: citizens, police_infringements, police_reports, actions_log, ingestion_runs, ingester idempotency
- [x] Dev seed + unit/integration tests (`tests.rs`)
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-police` + `gov-ingester-police` containers in `docker/phase1.yml`, DB healthcheck

### Stage 2 — Ingester (`services/gov-ingester-police`) — Rust ✅
- [x] Mock transport (JSON fixtures) + legacy transport stub + idempotent upsert

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/police/`) — TypeScript ✅
- [x] `actions.ts`, overview page, infringements (+ pay/dispute form), reports (+ form), `loading.tsx`

### Stage 4 — Staff View (`apps/portal-staff/app/dept/police/`) — TypeScript ✅
- [x] Read-only case worker view (overview / infringements / reports)

### Stage 5 — Federation ✅
- [x] OPA policy file (`policies/police.rego`), consent verification wired into `/citizen/data`

### Stage 6 — AI ✅
- [x] AI context (`police-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `PoliceDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` Police service + gateway + ingester env vars

---

## HUD — Full Native Module ✅

### Stage 1 — Dept Service (`services/gov-dept-hud`) — Rust ✅
- [x] `Cargo.toml`, `Dockerfile`, `src/` (main, error, db, routes, actions, consent, opa)
- [x] Migrations: citizens, hud_applications, hud_tenancies, hud_maintenance_requests, actions_log, ingestion_runs, ingester idempotency
- [x] Dev seed + unit/integration tests (`tests.rs`)
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-hud` + `gov-ingester-hud` containers in `docker/phase1.yml`, DB healthcheck

### Stage 2 — Ingester (`services/gov-ingester-hud`) — Rust ✅
- [x] Mock transport (JSON fixtures) + legacy transport stub + idempotent upsert

### Stage 3 — Portal UI (`apps/portal-citizen/app/dept/hud/`) — TypeScript ✅
- [x] `actions.ts`, overview page, applications (+ form), tenancy, maintenance (+ form), `loading.tsx`

### Stage 4 — Staff View (`apps/portal-staff/app/dept/hud/`) — TypeScript ✅
- [x] Read-only case worker view (overview / applications / tenancy / maintenance)

### Stage 5 — Federation ✅
- [x] OPA policy file (`policies/hud.rego`), consent verification wired into `/citizen/data`

### Stage 6 — AI ✅
- [x] AI context (`hud-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `HUDDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` HUD service + gateway + ingester env vars

---

## Corrections — Full Native Module ✅

### Stages 1–6 (`services/gov-dept-corrections` + `gov-ingester-corrections`) — Rust ✅
- [x] Dept service: `main.rs`, `error.rs`, `db.rs`, `routes.rs`, `actions.rs`, `consent.rs`, `opa.rs`, `tests.rs`
- [x] Ingester: mock + legacy transport, transform, idempotent upsert, tests
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-corrections` + `gov-ingester-corrections` containers in `docker/phase1.yml`

### Portal UI (`apps/portal-citizen/app/dept/corrections/`) — TypeScript ✅
- [x] `actions.ts`, overview page, case (+ request-summary form), probation, `loading.tsx`

### Staff View (`apps/portal-staff/app/dept/corrections/`) — TypeScript ✅
- [x] Read-only case worker view (overview / case / probation)

### Federation + AI ✅
- [x] OPA policy file (`policies/corrections.rego`), consent verification wired into `/citizen/data`
- [x] AI context (`corrections-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `CorrectionsDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` Corrections service + gateway + ingester env vars

---

## Customs — Full Native Module ✅

### Stages 1–6 (`services/gov-dept-customs` + `gov-ingester-customs`) — Rust ✅
- [x] Dept service: `main.rs`, `error.rs`, `db.rs`, `routes.rs`, `actions.rs`, `consent.rs`, `opa.rs`, `tests.rs`
- [x] Ingester: mock + legacy transport, transform, idempotent upsert, tests
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-customs` + `gov-ingester-customs` containers in `docker/phase1.yml`

### Portal UI (`apps/portal-citizen/app/dept/customs/`) — TypeScript ✅
- [x] `actions.ts`, overview page, declarations (+ submit form), travel, `loading.tsx`

### Staff View (`apps/portal-staff/app/dept/customs/`) — TypeScript ✅
- [x] Read-only case worker view (overview / declarations / travel)

### Federation + AI ✅
- [x] OPA policy file (`policies/customs.rego`), consent verification wired into `/citizen/data`
- [x] AI context (`customs-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `CustomsDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` Customs service + gateway + ingester env vars

---

## DOC — Full Native Module ✅

### Stages 1–6 (`services/gov-dept-doc` + `gov-ingester-doc`) — Rust ✅
- [x] Dept service: `main.rs`, `error.rs`, `db.rs`, `routes.rs`, `actions.rs`, `consent.rs`, `opa.rs`, `tests.rs`
- [x] Ingester: mock + legacy transport, transform, idempotent upsert, tests
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-doc` + `gov-ingester-doc` containers in `docker/phase1.yml`

### Portal UI (`apps/portal-citizen/app/dept/doc/`) — TypeScript ✅
- [x] `actions.ts`, overview page, concessions, permits (+ apply form), `loading.tsx`

### Staff View (`apps/portal-staff/app/dept/doc/`) — TypeScript ✅
- [x] Read-only case worker view (overview / concessions / permits)

### Federation + AI ✅
- [x] OPA policy file (`policies/doc.rego`), consent verification wired into `/citizen/data`
- [x] AI context (`doc-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `DOCDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` DOC service + gateway + ingester env vars

---

## MPI — Full Native Module ✅

### Stages 1–6 (`services/gov-dept-mpi` + `gov-ingester-mpi`) — Rust ✅
- [x] Dept service: `main.rs`, `error.rs`, `db.rs`, `routes.rs`, `actions.rs`, `consent.rs`, `opa.rs`, `tests.rs`
- [x] Ingester: mock + legacy transport, transform, idempotent upsert, tests
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-mpi` + `gov-ingester-mpi` containers in `docker/phase1.yml`

### Portal UI (`apps/portal-citizen/app/dept/mpi/`) — TypeScript ✅
- [x] `actions.ts`, overview page, certifications (+ apply form), registrations, `loading.tsx`

### Staff View (`apps/portal-staff/app/dept/mpi/`) — TypeScript ✅
- [x] Read-only case worker view (overview / certifications / registrations)

### Federation + AI ✅
- [x] OPA policy file (`policies/mpi.rego`), consent verification wired into `/citizen/data`
- [x] AI context (`mpi-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `MPIDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` MPI service + gateway + ingester env vars

---

## Stats NZ — Full Native Module ✅

### Stages 1–6 (`services/gov-dept-statsnz` + `gov-ingester-statsnz`) — Rust ✅
- [x] Dept service: `main.rs`, `error.rs`, `db.rs`, `routes.rs`, `actions.rs`, `consent.rs`, `opa.rs`, `tests.rs`
- [x] Ingester: mock + legacy transport, transform, idempotent upsert, tests
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-statsnz` + `gov-ingester-statsnz` containers in `docker/phase1.yml`

### Portal UI (`apps/portal-citizen/app/dept/statsnz/`) — TypeScript ✅
- [x] `actions.ts`, overview page, census, profile, `loading.tsx`

### Staff View (`apps/portal-staff/app/dept/statsnz/`) — TypeScript ✅
- [x] Read-only case worker view (overview / census / profile)

### Federation + AI ✅
- [x] OPA policy file (`policies/statsnz.rego`), consent verification wired into `/citizen/data`
- [x] AI context (`statsnz-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `StatsNZDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` Stats NZ service + gateway + ingester env vars

---

## Te Puni Kōkiri (TPK) — Full Native Module ✅

### Stages 1–6 (`services/gov-dept-tpk` + `gov-ingester-tpk`) — Rust ✅
- [x] Dept service: `main.rs`, `error.rs`, `db.rs`, `routes.rs`, `actions.rs`, `consent.rs`, `opa.rs`, `tests.rs`
- [x] Ingester: mock + legacy transport, transform, idempotent upsert, tests
- [x] Registered in root `Cargo.toml` workspace members
- [x] `gov-dept-tpk` + `gov-ingester-tpk` containers in `docker/phase1.yml`

### Portal UI (`apps/portal-citizen/app/dept/tpk/`) — TypeScript ✅
- [x] `actions.ts`, overview page, funding, programmes, `loading.tsx`

### Staff View (`apps/portal-staff/app/dept/tpk/`) — TypeScript ✅
- [x] Read-only case worker view (overview / funding / programmes)

### Federation + AI ✅
- [x] OPA policy file (`policies/tpk.rego`), consent verification wired into `/citizen/data`
- [x] AI context (`tpk-ai.ts`) + entitlement prompt (`ai-prompt.tsx`)

### Cross-cutting wiring ✅
- [x] `TPKDataBundle` schema (`packages/@tpt/gov-schema`)
- [x] Portal `DeptId` / `DEPARTMENTS` / `services` registry
- [x] `.env.example` TPK service + gateway + ingester env vars

---

### Staff Portal (`apps/portal-staff`) — TypeScript
- [x] Next.js 15 scaffold (+ IRD staff views: overview, tax-summary, GST, KiwiSaver, WFF)
- [x] Staff authentication (login page + staff session JWT, demo + shared-passphrase real mode, route-protecting middleware)
- [x] Citizen search (consent-gated) — `/citizens` search + per-department consent check against identity server grants
- [x] Cross-dept case view, case notes, referral flow — `/citizens/[did]` aggregates consented dept bundles, case notes + referrals persisted locally; department detail pages consent-gated

### Production Hardening
- [x] RealMe SAML2 integration (production-grade) — `apps/portal-staff/app/lib/realme.ts`:
  - Signed Redirect-binding `AuthnRequest` (RSA-SHA256 query-string signature), `InResponseTo` correlation via signed state cookie
  - SP metadata route `/realme/metadata` + Single Logout stub `/login/realme/slo`
  - Conformant POST-binding response verification via `xml-crypto` (Exclusive C14N, enveloped-signature transform, digest + signature, **pinned IdP cert**, anti-signature-wrapping `validateElementAgainstReferences`)
  - Response validation: status, conditions (NotBefore/NotOnOrAfter + clock skew), audience restriction, `InResponseTo`, issuer; replay protection on `Assertion` ID
  - `scripts/gen-realme-sp-certs.sh` for dev SP key/cert; env wiring in `.env.example`. Live IdP cert + entityID still required for production (see `auth-actions.ts`). Unit tests in `app/lib/realme.test.ts` (12 cases)
- [x] mTLS between all internal services — `crates/gov-mtls` (rustls server/client configs, internal CA, `scripts/gen-mtls-certs.sh`)
- [x] QUIC transport in `gov-federation-node` — `quic.rs`, gated behind `quic` feature
- [x] ZK-SNARK age/residency proofs — `crates/gov-zk` (Schnorr/OR-proof range proofs over Ristretto; no trusted setup, demo-grade/unaudited)
- [x] Schema registry — `services/gov-schema-registry` (Rust + Postgres; register/resolve/validate schemas), `SchemaRegistryClient` in `@tpt/gov-schema`, container in `docker/phase1.yml`
- [x] Load test (k6) — `load-tests/k6/` (gateway, identity, dept-ird)
- [ ] External security audit
- [x] te reo Māori opt-in toggle

---

## Phase 3 — Full Vision

- [x] Remaining NZ Public Service departments + major Crown entities (each Stages 1–6). Departments already covered in Phase 1/2 (IRD, WINZ/MSD, MOH, DIA, NZTA, ACC, MoE/NZQA, MBIE, LINZ) are excluded below. Each listed dept below now has a Rust dept service + ingester, a `DataBundle` schema (`packages/@tpt/gov-schema`), an OPA policy (`policies/`), and citizen + staff portal UIs.
  - **Central government**
    - [x] The Treasury — economic/fiscal data, Budget (`gov-dept-treasury`)
    - [x] Department of the Prime Minister and Cabinet (DPMC) (`gov-dept-dpmc`)
    - [x] Te Kawa Mataaho Public Service Commission (`gov-dept-publicservice`)
    - [x] Stats NZ — census, profile data (see "Stats NZ — Full Native Module" above)
    - [x] Crown Law Office (`gov-dept-crownlaw`)
  - **Justice & safety**
    - [x] Ministry of Justice — fines, Disputes Tribunal, name changes, court records (see "MOJ — Full Native Module" below)
    - [x] New Zealand Police — infringements, disputes (see "Police — Full Native Module" below)
    - [x] Department of Corrections — case management, probation (see "Corrections — Full Native Module" above)
    - [x] Serious Fraud Office (`gov-dept-sfo`)
    - [x] New Zealand Customs Service — traveller declarations (see "Customs — Full Native Module" above)
  - **Social & community**
    - [x] Oranga Tamariki — Ministry for Children (`gov-dept-oranga`)
    - [x] Ministry of Housing and Urban Development / Kāinga Ora — social housing (see "HUD — Full Native Module" below)
    - [x] Ministry for Women (`gov-dept-women`)
    - [x] Ministry for Pacific Peoples (`gov-dept-pacific`)
    - [x] Ministry for Ethnic Communities (`gov-dept-ethnic`)
    - [x] Ministry of Māori Development (Te Puni Kōkiri) — funding, programmes (see "Te Puni Kōkiri (TPK) — Full Native Module" above)
    - [x] Te Arawhiti (Office for Māori Crown Relations) (`gov-dept-tearawhiti`)
  - **Economic & regulatory**
    - [x] Ministry for Regulation (`gov-dept-regulation`)
    - [x] Immigration New Zealand — visas (part of MBIE; cross-ref Phase 2 MBIE line)
    - [x] WorkSafe New Zealand (`gov-dept-worksafe`)
    - [x] Companies Office (part of MBIE)
    - [x] Retirement Commission (Te Ara Ahunga Ora) (`gov-dept-retirement`)
  - **Environment & primary industries**
    - [x] Ministry for Primary Industries (MPI) — certifications, registrations (see "MPI — Full Native Module" above)
    - [x] Ministry for the Environment (MfE) (`gov-dept-mfe`)
    - [x] Department of Conservation (DOC) — concessions, permits (see "DOC — Full Native Module" above)
    - [x] Earthquake Commission — Toka Tū Ake (EQC) (`gov-dept-eqc`)
  - **Transport & infrastructure**
    - [x] Ministry of Transport (`gov-dept-mot`)
    - [x] Civil Aviation Authority (`gov-dept-caa`)
    - [x] Maritime New Zealand (`gov-dept-maritime`)
    - [x] Fire and Emergency New Zealand (FENZ) (`gov-dept-fenz`)
  - **Education & culture**
    - [x] Ministry of Education (core, beyond NZQA already in Phase 2) (`gov-dept-moe`)
    - [x] Education Review Office (ERO) (`gov-dept-ero`)
    - [x] Tertiary Education Commission (`gov-dept-tec`)
    - [x] Ministry for Culture and Heritage (`gov-dept-mch`)
  - **Foreign affairs & defence**
    - [x] Ministry of Foreign Affairs and Trade (MFAT) (`gov-dept-mfat`)
    - [x] Ministry of Defence (`gov-dept-defence`)
    - [x] New Zealand Defence Force (`gov-dept-nzdf`)
    - [x] Government Communications Security Bureau (GCSB) (`gov-dept-gcsb`)
    - [x] New Zealand Security Intelligence Service (NZSIS) (`gov-dept-nzsis`)
  - [ ] + any further agencies identified
- [x] `apps/portal-policy` — AI policy simulation (`apps/portal-policy`, Next.js 15; scenario catalog + AI cross-dept impact simulation via `GovAiClient`)
- [ ] AI level 3 (automated) — routine benefit renewals, document classification
- [x] Sovereign Ollama deployment guide (government hardware) — `docs/sovereign-ollama.md`
- [ ] Local council federation (opt-in)
- [x] ISO 27001 compliance documentation — `docs/iso-27001-compliance.md` (control mapping + gap register)
- [x] International fork guide — `docs/international-fork-guide.md`
- [ ] Optional Māori data sovereignty layer (iwi-controlled nodes)

---

## Ongoing / Cross-cutting

- [x] `packages/@tpt/gov-ui` — shared design system (TypeScript / React)
- [x] `packages/@tpt/gov-i18n` — i18n strings (en-NZ + te reo Māori opt-in)
- [x] CI pipeline (`.github/workflows/ci.yml`) — TS (lint/typecheck/test) + Rust (fmt, clippy, build, unit tests) jobs. Rust integration tests (needing a DB) excluded from CI — run locally via `cargo test --workspace`
- [x] `LICENSE-MIT` + `LICENSE-APACHE` — dual MIT OR Apache-2.0
- [x] `README.md` — overview, quick start, architecture diagram
- [x] `CONTRIBUTING.md` — how to add a new department (Stages 1–6)

---

## Platform Review Roadmap (2026-11) — hybrid direction

Source: full platform review. Direction: **hybrid** — Rust owns correctness-critical logic (auth, sessions, consent, audit, schemas); Next.js stays a thin UI layer. Items marked *(verify)* were found by reading code only and need confirming before fixing.

### P0 — Security & correctness (before any pilot)

**Rust**
- [x] Persistent identity-server issuer key (`gov-identity-server/src/routes.rs` `issue_grant` currently calls `CredentialIssuer::generate()` per request); expose public key endpoint
- [x] Dept services verify `DataGrantCredential` signatures — every dept service's `verify_access` path now loads the issuer key via `gov-consent` and fails closed (`enforce_signed_and_unrevoked` in each `consent.rs`; OPA can only restrict). Applied to the template and all 46 copies, plus `gov-dept-ird` and the gateway already wired. Config: `TPT__GOV__ISSUER_PUBLIC_KEY` + `TPT__GOV__IDENTITY_API_KEY` added to `docker/phase1.yml` and `.env.example`. *Caveat: 7 patched services type-check clean (acc, caa, corrections, dia, moh, nzta, winz); 39 others fail to compile for pre-existing reasons (see "Dept route compile errors" below), so their new code is unverified by the compiler.*
- [x] Enforce revocation in dept services — same helper calls `RevocationClient::ensure_active` on every presented grant id (fail-closed); signature and scope checked on the same grant.
- [x] Dept services compile against a real database (fixed generator output, not yet the generator itself): all 47 dept services with migrations now `cargo check` clean against a local Postgres 16 container, with per-service databases migrated from `services/<dept>/migrations`. Fixes applied:
  - `"key": c.c` / `c.p` placeholders in `routes.rs` mapped to the row struct fields (the compiler's "available fields" note drove the mapping).
  - `Some(db::fetch_x(...))` wrapping `Option`-returning fetches removed (statsnz, eqc, moe, retirement, treasury, fenz).
  - Domain tables missing from migrations created in `092_domain_tables.sql` for 27 services, with the seed moved to `094_seed_dev.sql`.
  - Duplicate migration files removed in `gov-dept-moj` (`002_fines.sql`, `003_disputes.sql`, `004_court_records.sql`; the `_moj_` variants match the row types).
  - `Decimal::to_f64` replaced with a string parse in `gov-dept-msd` (no trait import needed).
  - The generator (`scripts/gen-phase3-*.mjs`) now emits the same fixes: placeholder mapping, no `Some()` around `Option` fetches, entity migrations written, and cross-department consent enforcement with `gov-consent`. Verified by regenerating `crownlaw` (list entities) and `eqc` (single-record entity) into a scratch worktree, formatting, migrating a fresh database and running `cargo check` on each with no manual edits.
  - *Still open: regenerating a whole department also rewrites portal, ingester, policy and schema files that have drifted from the generator, about 50 files for crownlaw. Do not regenerate a department in place until those templates are reconciled. `sqlx` offline caches were not regenerated.*
- [ ] Remove `requesting_dept_id` self-assertion bypass (`is_direct_access`); gateway sets caller identity from JWT, JWT `sub` must equal `req.did`, enforce `Claims.scopes`. *(Partial: the gateway already sets `x-requesting-dept-id` from the JWT and drops caller identity headers. Open: department nodes still treat a request without that header as direct access. Decided in `docs/adr/0002-department-node-access-control.md` (mTLS from gateway and staff portal). Not implemented.)*
- [x] Authenticate identity-server endpoints (`register_did`, `issue_grant`, `revoke_grant`, `list_grants`); make DID registration first-write-wins (no key overwrite); validate dept ids, scopes, `expires_in_seconds` bounds
- [x] Axum 0.8 route syntax in `gov-identity-server/src/main.rs` (`:did`/`:id` → `{did}`/`{id}`) *(verify with `cargo run`)*
- [x] `gov-dept-node/src/main.rs` — REST routes unreachable because `fed_node.start()` blocks; run federation and HTTP concurrently
- [ ] Audit: fail request when audit write fails; append-only table (revoke UPDATE/DELETE + hash chain); produce signed `AuditLogEntry`; IRD read audit; real `citizen_did` on actions *(partial: fail-closed write + hash chain + signed entry in `gov-dept-node`; IRD read audit, real action `citizen_did`, and UPDATE/DELETE revocation still open)*
- [ ] Federation: envelope replay protection (`message_id` dedup, timestamp freshness, `to_dept_id` check), persistent node keys, resolve `consent_grant_ids`, QUIC client auth (mTLS), per-peer rate limit *(partial: replay/rate-limit/registry structs added but not wired into `receive_envelope`; timestamp/`to_dept_id` checks and QUIC mTLS still open)*
- [x] Gateway: fail closed when `TPT__GOV__JWT_SECRET` unset; forward Authorization; shared `reqwest::Client`; upstream timeouts and body limits; real `citizen_resolve`; per-caller rate limit; bound the rate-limit map; replace mutex `.unwrap()` (circuit breaker, rate limiter); don't echo JWT error text

**TypeScript / portals** (largely superseded by the Rust auth/BFF service below, but fix the quick wins now)
- [x] `portal-citizen/app/lib/data-access.ts:25-26` syntax error and `STATSNZDataBundle` vs `StatsNZDataBundle` export mismatch (breaks typecheck) *(verify)*
- [x] Citizen login: bind public key to DID and register with proof. Sign-in now starts with an identity-server challenge bound to the DID and key; the wallet signs the decoded challenge bytes; the verify route checks the signature and then calls `/v1/did/register` with proof, and denies sign-in if registration fails. Verified end to end against a live identity server and Postgres (9/9 checks, using a scratch script that is not committed). Fixed on the way: identity server `register-with-proof` read a non-existent `_id` column.
- [x] Staff portal: verify JWT in middleware and in write actions. Middleware verifies the HS256 signature and expiry with Web Crypto on every non-public request (tested against tokens from the Node signer). Case-note and referral writes call `requireStaffSession()`. Read-only dept pages rely on the middleware check.
- [ ] RealMe: fail closed without `REALME_IDP_CERT`; remove/guard `REALME_SKIP_VERIFY`; staff allow-list/roles; SubjectConfirmationData (Recipient, NotOnOrAfter, InResponseTo required), Response Issuer, `SessionIndex` from `AuthnStatement`; harden xmldom (no DOCTYPE/entities); persistent replay store; signed SLO / CSRF-safe logout; validate `relayState`
- [x] Remove hard-coded fallback session secrets (both portals); throw in production *(throws on first `sessionSecret` access, not at process start; dev fallback retained when `NODE_ENV !== "production"`)*
- [x] Demo login server actions must check demo mode server-side (`loginDemo`, `setScenario`, `loginStaffDemo`); constant-time staff password compare; per-account staff login rate limit (5 failures / 15 min, in-process; a shared store is needed for multiple instances).
- [ ] AI: `/api/ai` use session `grantedScopes`, bound input, guard `req.json()`, redact context chunks (medical/WINZ data), don't leak upstream errors; authenticate + rate-limit `portal-policy` AI actions; align `AI_*` vs `TPT__GOV__AI_*` env names and defaults *(partial: `/api/ai` done — fetches only granted depts/scopes, 2000-char cap, JSON guard, context chunks redacted, upstream errors logged not returned. Open: `portal-policy` AI auth + rate limit, env-name alignment)*

### Hybrid architecture
- [ ] Rust auth/BFF service (new, or folded into `gov-gateway`): sessions/JWT, DID-to-key binding, RealMe SAML validation, consent + audit; portals call it via one shared guard
- [ ] Schemas from Rust: `serde` + `schemars`/`utoipa` → JSON Schema/OpenAPI → generated TS types/clients for `@tpt/gov-schema` consumers
- [ ] Collapse 48 near-identical dept services into one config-driven generic dept node + per-dept data modules; generator emits config + data modules
- [ ] (Deferred) full-Rust UI trial on `portal-open-data` only

### P1 — Gaps & drift
- [x] Remove stale workspace refs: `pnpm-workspace.yaml` (`services/gov-ai`, `services/gov-analytics`, `packages/adapters/*`), `portal-citizen/tsconfig.json` `@tpt/adapter-ird` alias, `ird/ai-context.ts` comment; reconcile `CLAUDE.md` / `CONTRIBUTING.md` / `TODO.md` (adapter model vs native modules); fix Windows path in `docs/international-fork-guide.md:28`. *(Done: `CLAUDE.md` and `README.md` now describe native department services. `CONTRIBUTING.md` already did.)*
- [x] Staff registry: add `hud`, `moj`, `police`; dedupe `StaffDeptId` *(done in `apps/portal-staff/app/lib/config.ts`; `StaffBundle` extension beyond 16 depts still open)*
- [ ] Zod-validate client input in ~47 dept `actions.ts`; unify demo flag with `PORTAL_CONFIG`
- [ ] Replace local-JSON case notes (`apps/portal-staff/app/lib/case-notes.ts`) with audited DB storage
- [ ] Tests: identity server; HTTP-level forged/unsigned/revoked grant rejection; direct-access bypass; portal auth/JWT/consent/AI route; OpenRouter/Ollama providers; staff middleware; RealMe fail-open path. *(Partial: citizen tests cover DID binding and Ed25519 verification; staff tests cover the lockout limiter, the Edge JWT verifier and the RealMe metadata; OPA tests for `ird.rego` and an example policy run in CI; end-to-end sign-in verified by script. Still open: Rust HTTP-level grant tests, OpenRouter/Ollama tests, consent and AI route tests.)*
- [ ] ESLint config; per-package tsconfigs for `gov-ai-client`, `gov-i18n`; apps extend `tsconfig.base.json`. *(Partial: all three portals plus `portal-policy` now have a flat ESLint config (`next/core-web-vitals` + TypeScript); `pnpm lint` passes non-interactively. The citizen and staff portals have `test` scripts (vitest). Per-package tsconfigs are still open.)*
- [x] Security headers: nonce-based CSP per request (`script-src` has no `unsafe-inline`), `frame-ancestors 'none'`, `connect-src 'self'`, HSTS, Permissions-Policy, in both portals. Verified on the citizen production build (all script tags carry the nonce). *Open: styles still allow `unsafe-inline` for React inline styles. Cookie `secure` now defaults on; set `TPT__GOV__COOKIE_SECURE=false` only for plain-HTTP testing.*
- [ ] Accessibility: skip link and `<nav aria-label>` in staff portal *(done)*; focus trap, Escape and `aria-live` in `AiChatWidget` *(done)*; RealMe button contrast (≥4.5:1) *(open)*; reduce inline styles *(open)*
- [ ] te reo Māori: extend i18n beyond 9 files; staff + policy portal i18n; server-rendered `lang`

### P2 — Adoption accelerators
- [ ] Public department generator: rename `scripts/gen-phase3-*.mjs`, `pnpm new:dept <id> --spec dept.yaml`, `--dry-run`, compile check, workspace patching, docs
- [ ] `examples/` + `templates/`: minimal dept, sample Rego consent policy, `DataGrantCredential` flow, "agency in a box" compose. *(Partial: `examples/` has the consent flow, a sample policy and its tests. `templates/` and the compose file are open.)*
- [ ] Root `Makefile`/`justfile` (`demo`, `stack`, `seed`, `test`, `new-dept`) + `pnpm doctor`; group `.env.example` required/optional; env validator. *(Partial: `Makefile` (no `new-dept`, deliberately: the generator rewrites files) and `pnpm doctor` are in place; `.env.example` has a section for consent and cookie settings. Still open: `seed`, grouping of required and optional variables, an env validator.)*
- [ ] Prebuilt portal images (multi-stage Dockerfiles, GHCR); Helm/Kustomize; secrets-management guide; NZ hosting runbook
- [ ] `demo-full`: one persona across all depts showing federation, OPA consent denial, audit trail; hosted demo + guided tour
- [ ] Unified seed/reset command and shared fictional persona fixtures (only 4 SQL files in `docker/init/`)
- [ ] OpenAPI per service (`utoipa`), merged at gateway; generated TS client; contract tests
- [x] Repo hygiene: `SECURITY.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `CHANGELOG.md`, `NOTICE`, issue templates, `CODEOWNERS`, Dependabot; removed tracked `history/` and `.mimocode/`. *No PR template: the project accepts issues only (see `CONTRIBUTING.md`). `CODEOWNERS` assigns `@PhillipC05`; confirm the maintainer handle.*
- [x] NZ compliance pack `docs/compliance/`: NZISM mapping, Privacy Act 2020 IPPs, PIA template, breach runbook, Māori data sovereignty, Algorithmic Charter statement, data residency and classification, threat model and SBOM plan. Every control is labelled implemented, scaffold or planned. *Open: the ISO 27001 document still needs its overstated claims corrected, and legal and assessor review is required before any use.*

### P3 — Automation / CI / observability
- [ ] CI: run `pnpm lint` + `pnpm build`; Postgres service container + Rust integration tests; `opa test`/`opa check` on `policies/*.rego`; `cargo audit`/`cargo-deny`, `pnpm audit`, CodeQL, gitleaks; `docker/demo.yml` smoke test; generator drift check; Playwright e2e (login, consent, forged cookie, unsigned SAML); Linux + Windows matrix. *(Partial: CI now runs lint, typecheck, tests, portal builds, `opa check`/`opa test` over `policies` and `examples`, and gitleaks. Still open: Postgres service and integration tests, cargo audit, CodeQL, the demo smoke test, the drift check, Playwright and the OS matrix.)*
- [ ] Observability: readiness vs liveness (gateway reports downstream health), `/metrics` Prometheus, cross-dept trace propagation, Grafana dashboard + alerts, audit-log export for SIEM

### P4 — Innovation
- [ ] Citizen "who looked at my data" timeline with plain-language reasons
- [ ] Consent receipts + one-tap cross-dept revoke, expiry nudges
- [ ] Policy simulation in CI: replay Rego changes on synthetic data, diff allow/deny
- [ ] Verifiable audit anchoring (periodic Merkle root to a public log)
- [ ] Life-event journeys (new baby, job loss, moving) with a single consent
- [ ] Synthetic-persona generator for demos/load tests; agency readiness score from `pnpm doctor`
- [ ] Tikanga-aware consent / iwi-controlled nodes (links to existing Māori data sovereignty item)

---

## Multi-Repo Platform Split (2026-11)

> Direction: platform core repo + `tpt-gov-dept-template` + one repo per department. **Finish and freeze the core in this monorepo first; split only once the exit criteria below are met.** A monorepo can't restrict read access per department, blurs ownership, and can't hold classified agencies (NZSIS, GCSB, Defence), so it is a temporary workspace, not the end state.

### Core-done exit criteria (trigger for splitting)
- [ ] `gov-schema`, federation envelope and `DataGrantCredential` format frozen at v1.0 with semver + `CHANGELOG.md` (additive-only within a major)
- [ ] Identity server production-ready (persistent issuer key + rotation, authenticated endpoints, migrations) — see P0 items above
- [ ] Gateway, mTLS/QUIC, immutable audit trail and OPA consent enforcement complete and tested — see P0 items above
- [ ] Core crates and `@tpt/*` packages publishable to a registry (crates.io/private registry, npm); no path deps from departments
- [ ] Runtime department registry: manifest per dept (id, endpoints, scopes, UI module URL); portal has no compile-time imports of `app/dept/<id>/`
- [ ] Conformance suite (`conformance/`): signed envelopes, consent VC verify + revocation, scope enforcement, audit written on both sides, replay rejection
- [ ] One dept (IRD) passes the conformance suite using only published platform versions

### Repo layout (target)
- [ ] `tpt-gov-platform` — crates, gov-schema, gov-ai-client, gov-ui, gov-i18n, gateway, identity server, schema registry, shared OPA policies, conformance suite
- [ ] `tpt-gov-dept-template` — GitHub template repo built from `gov-dept-node` + one ingester (node, ingester, OPA policy, schema, CI, portal UI module)
- [ ] `tpt-dept-<id>` — one repo per department, generated from the template, pins platform versions (private/air-gapped option for security agencies)
- [ ] `tpt-gov-portals` — citizen / staff / policy / open-data portals, discover dept UI at runtime
- [ ] `tpt-gov-deploy` — Helm/Terraform/Compose, environment config, release trains

### Migration steps
- [ ] Decouple portal from dept code (registry-driven loading, module federation or schema-driven renderer)
- [ ] Collapse near-identical dept services into the config-driven template (links to "Collapse 48 near-identical dept services" above)
- [ ] Add semver, release automation and a compatibility matrix for platform packages
- [ ] Pilot split: extract IRD, then WINZ, MOH, DIA; validate versioning and CI across repos
- [ ] Script generation of the remaining dept repos from the template; move portals and deploy config to their own repos
- [ ] Governance per repo: `CODEOWNERS` per agency, `SECURITY.md`, ADRs, release/branch policy, contribution guide for agency teams

### Capability-based security (design BEFORE the v1.0 contract freeze)
> Adopt the capability *model* from `tpt-archon`, not the crate: Archon's `Capability` (`tpt-archon-bridge/src/capability.rs`) is in-process only (private constructor, `Rc<RefCell>`, no signature/serialisation/attenuation), and Archon is not production-ready. Several P0 items above (`requesting_dept_id` bypass, unenforced JWT scopes, per-request issuer key, ambient DB access) are ambient-authority problems that capabilities remove.
- [x] ADR: capability token format — decided in `docs/adr/0001-capability-token-format.md`: extend `DataGrantCredential` (Ed25519, holder binding, typed caveats, attenuation by signed parent reference). Biscuit, UCAN and macaroons were considered and rejected. Implementation is open.
- [ ] Evolve `DataGrantCredential` (`crates/gov-identity-core/src/credential.rs`) into a signed, holder-bound capability: resource (dept/dataset/fields), rights, caveats (expiry, purpose, audience, max uses), proof-of-possession of the holder key
- [ ] Delegation by attenuation only (a derived token can narrow, never widen); verify the full chain
- [ ] Short TTL + revocation list; enforce revocation at every dept node
- [ ] Dept nodes accept only capabilities (no ambient authority); per-scope DB roles + Postgres row-level security; gateway/portals hold no data credentials
- [ ] Single shared verification path (`consent::verify_access` with the persistent issuer key) used by all dept services/generator template
- [ ] Sandbox ingesters with `tpt-linux-sandbox` (Landlock/seccomp: network only to the legacy source, write only to the dept DB)
- [ ] Run plugins / AI tools in `tpt-wasm` with an explicit capability host; use `tpt-privd` for privileged operations
- [ ] `gov-ai-client` receives a read-only, purpose-bound, attenuated capability — never the raw grant
- [ ] Audit: every capability use writes a signed `AuditLogEntry` including the delegation chain
- [ ] Conformance tests: forged, widened-by-delegation, wrong-holder, expired and revoked capabilities all rejected
- [ ] (Later) Evaluate `tpt-archon` as an optional capability-checked dept datastore once it is production-ready (WAL concurrency, pg wire completeness, machine-checked proofs)
