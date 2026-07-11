# Contributing

Thanks for your interest in **tpt-gov-nz**. This guide explains how the codebase is
organised and how to add a new government department end-to-end.

## Ways of working

- **TypeScript** is used for portals, shared packages, i18n and the UI.
- **Rust** is used for services, ingesters, the gateway, the identity server and
  federation. It is a single Cargo workspace at the repo root.
- Every department follows the same six-stage pattern. New departments go
  **straight to native modules** — we no longer write `DeptAdapter` implementations
  (the IRD adapter is the only one left, and it is being phased out).

## Adding a department (Stages 1–6)

Pick a `dept_id` (e.g. `nzta`). Then:

### Stage 1 — Department service (Rust)

1. Add `services/gov-dept-<id>/` to the root `Cargo.toml` workspace members.
2. Scaffold an Axum server with a SQLx pool that runs migrations on startup.
3. Implement the standard routes:
   - `GET /health`
   - `POST /citizen/resolve` — resolve a DID to an internal citizen id
   - `POST /citizen/data` — return the data bundle **for the requested scopes**,
     after consent verification (see Stage 5)
   - `POST /citizen/action` — perform an action on behalf of the citizen
4. Add SQL migrations for the citizen/entity tables, an `actions_log`, and a dev
   seed.
5. Write unit + integration tests (resolve found/not-found, `fetch_data` per scope
   combination, actions valid/invalid).

### Stage 2 — Ingester (Rust)

- `services/gov-ingester-<id>/` with a transport trait. Provide a **mock transport**
  reading JSON fixtures (so the stack runs with no legacy system), plus a stub for
  the real legacy transport. Transform → idempotent upsert. Add a scheduler.

### Stage 3 — Portal UI (TypeScript)

- In `apps/portal-citizen/app/dept/<id>/`:
  - `actions.ts` — `fetch<Id>Data(scopes)` and `submit<Id>Action(type, params)`
    server actions. Call `gov-dept-<id>` directly (no adapter).
  - `page.tsx` overview + sub-pages for the key entities.
  - Build AI context inline (do **not** add an adapter) and reuse
    `askWithContext` from `@tpt/gov-ai-client`.
- Register the department in `apps/portal-citizen/app/lib/config.ts` (`DEPARTMENTS`)
  so it appears on the dashboard and in the consent matrix.

### Stage 4 — Staff view (TypeScript)

- Read-only case-worker views in `apps/portal-staff/app/dept/<id>/` (same data,
  no action buttons).

### Stage 5 — Federation

- Write `policies/<id>.rego` (OPA consent policy).
- Wire consent verification into `gov-dept-<id>`'s `/citizen/data` (OPA call with a
  local fallback), and write a cross-department request test (with and without a
  grant).

### Stage 6 — AI

- Add an AI prompt page/component ("Am I eligible for X?", "What do I do about Y?").
- For AI *action suggestions* (level ≥ `assisted`), add a pure helper and keep it
  out of any `"use server"` module (server-action modules may only export async
  functions — extract pure helpers into a separate file).

## Shared contracts

- Add or change data shapes in `packages/@tpt/gov-schema/` (Zod schemas + types).
  Both Rust (via generated JSON) and TypeScript import these.
- Keep PII redaction in `packages/@tpt/gov-ai-client` (NHI, IRD number, passport,
  phone).

## i18n

- Add user-facing strings to `packages/@tpt/gov-i18n` for both `en-NZ` and
  `te reo Māori`. The portal reads these via the `LanguageProvider`.

## Consent & privacy

- Never centralise a citizen profile. Departments serve their own data.
- Every cross-department access needs an explicit, signed, auditable grant.
- The AI assistant only ever receives consented, PII-redacted context.

## Before opening a PR

```bash
pnpm install
pnpm turbo typecheck lint test      # TypeScript workspaces
cargo test --workspace              # Rust workspace
```

CI runs the same checks (see `.github/workflows/ci.yml`).
