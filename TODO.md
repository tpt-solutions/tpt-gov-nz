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
- [ ] **MoE / NZQA** — qualifications, transcripts
- [ ] **MSD** — student loans/allowances (StudyLink), broader case history beyond Work and Income (see "WINZ — Full Native Module" above)
- [ ] **MBIE** — business registrations, company director lookup
- [ ] **LINZ** — property titles, land ownership

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

### Staff Portal (`apps/portal-staff`) — TypeScript
- [x] Next.js 15 scaffold (+ IRD staff views: overview, tax-summary, GST, KiwiSaver, WFF)
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
- [x] te reo Māori opt-in toggle

---

## Phase 3 — Full Vision

- [ ] Remaining NZ Public Service departments + major Crown entities (each Stages 1–6). Departments already covered in Phase 1/2 (IRD, WINZ/MSD, MOH, DIA, NZTA, ACC, MoE/NZQA, MBIE, LINZ) are excluded below.
  - **Central government**
    - [ ] The Treasury — economic/fiscal data, Budget
    - [ ] Department of the Prime Minister and Cabinet (DPMC)
    - [ ] Te Kawa Mataaho Public Service Commission
    - [ ] Stats NZ — census, official statistics (likely feeds `portal-open-data` rather than a citizen dept module)
    - [ ] Crown Law Office
  - **Justice & safety**
    - [ ] Ministry of Justice — fines, Disputes Tribunal, name changes, court records
    - [ ] New Zealand Police — infringements, disputes
    - [ ] Department of Corrections
    - [ ] Serious Fraud Office
    - [ ] New Zealand Customs Service — traveller declarations
  - **Social & community**
    - [ ] Oranga Tamariki — Ministry for Children
    - [ ] Ministry of Housing and Urban Development / Kāinga Ora — social housing
    - [ ] Ministry for Women
    - [ ] Ministry for Pacific Peoples
    - [ ] Ministry for Ethnic Communities
    - [ ] Ministry of Māori Development (Te Puni Kōkiri)
    - [ ] Te Arawhiti (Office for Māori Crown Relations)
  - **Economic & regulatory**
    - [ ] Ministry for Regulation
    - [ ] Immigration New Zealand — visas (part of MBIE; cross-ref Phase 2 MBIE line)
    - [ ] WorkSafe New Zealand
    - [ ] Companies Office (part of MBIE)
    - [ ] Retirement Commission (Te Ara Ahunga Ora)
  - **Environment & primary industries**
    - [ ] Ministry for Primary Industries (MPI)
    - [ ] Ministry for the Environment (MfE)
    - [ ] Department of Conservation (DOC)
    - [ ] Earthquake Commission — Toka Tū Ake (EQC)
  - **Transport & infrastructure**
    - [ ] Ministry of Transport
    - [ ] Civil Aviation Authority
    - [ ] Maritime New Zealand
    - [ ] Fire and Emergency New Zealand (FENZ)
  - **Education & culture**
    - [ ] Ministry of Education (core, beyond NZQA already in Phase 2)
    - [ ] Education Review Office (ERO)
    - [ ] Tertiary Education Commission
    - [ ] Ministry for Culture and Heritage
  - **Foreign affairs & defence**
    - [ ] Ministry of Foreign Affairs and Trade (MFAT)
    - [ ] Ministry of Defence
    - [ ] New Zealand Defence Force
    - [ ] Government Communications Security Bureau (GCSB)
    - [ ] New Zealand Security Intelligence Service (NZSIS)
  - [ ] + any further agencies identified
- [ ] `apps/portal-policy` — AI policy simulation
- [ ] AI level 3 (automated) — routine benefit renewals, document classification
- [ ] Sovereign Ollama deployment guide (government hardware)
- [ ] Local council federation (opt-in)
- [ ] ISO 27001 compliance documentation
- [ ] International fork guide
- [ ] Optional Māori data sovereignty layer (iwi-controlled nodes)

---

## Ongoing / Cross-cutting

- [x] `packages/@tpt/gov-ui` — shared design system (TypeScript / React)
- [x] `packages/@tpt/gov-i18n` — i18n strings (en-NZ + te reo Māori opt-in)
- [x] CI pipeline (`.github/workflows/ci.yml`) — TS (lint/typecheck/test) + Rust (fmt, clippy, build, unit tests) jobs. Rust integration tests (needing a DB) excluded from CI — run locally via `cargo test --workspace`
- [x] `LICENSE` — Apache 2.0
- [x] `README.md` — overview, quick start, architecture diagram
- [x] `CONTRIBUTING.md` — how to add a new department (Stages 1–6)
