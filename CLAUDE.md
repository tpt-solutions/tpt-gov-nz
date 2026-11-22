# tpt-gov-nz

Unified NZ government digital platform — open source, federated, AI-optional.

## Architecture

Hybrid monorepo: TypeScript (Turbo + pnpm) for portals and packages; Rust (Cargo workspace) for systems-level services and department nodes.

```
apps/            # Next.js 15 portals (portal-citizen, portal-staff, portal-policy, portal-open-data)
services/        # Long-running services
  gov-gateway/   # Rust/Axum API gateway
  gov-identity-server/  # DID + VC issuance
  gov-dept-node/ # Template each department service is built from
  gov-dept-<id>/ # Native department service: owns its database, serves data to portals and federation
  gov-ingester-<id>/    # Pulls from a department's legacy system into its database
crates/          # Rust libraries
  gov-consent/          # Grant signature + revocation verification shared by department services
  gov-federation-core/  # Protocol types, Ed25519, envelopes
  gov-federation-node/  # QUIC peer runtime
  gov-identity-core/    # DID, Verifiable Credentials
packages/
  @tpt/gov-schema/      # Zod schemas — the single source of truth for types
  @tpt/gov-ai-client/   # Provider-agnostic AI (OpenRouter / Ollama)
docker/          # Docker Compose for local dev
```

## Key design rules

1. **Department services are called directly.** Each `gov-dept-<id>` owns its database and is called by the portal without an adapter layer. The portal's department modules live in `apps/portal-citizen/app/dept/<id>/`. Shared behaviour is in `services/gov-dept-node` and `crates/gov-consent`.
2. **No central PII store** — citizen profile is assembled at query time from federated dept nodes. Identity server holds only public keys + VC metadata.
3. **AI is always optional** — `GovAiClient` with `level: "none"` is a no-op. Platform must work fully without any AI configured.
4. **Consent gates all data sharing** — `DataGrantCredential` (signed VC) must accompany every `DATA_REQUEST`. OPA sidecar on the receiving node verifies it.
5. **Audit trail is immutable** — both sides of every data exchange write a signed `AuditLogEntry`. Citizens can view their full audit trail.

## AI levels

| Level | Behaviour |
|-------|-----------|
| `none` | AI disabled everywhere |
| `advisory` | AI surfaces suggestions; humans decide |
| `assisted` | AI drafts; human approves before action |
| `automated` | AI executes routine actions with full audit + human override |

Set globally via `TPT__GOV__AI_LEVEL` env var. Override per department in dept node config.

## AI providers

- `openrouter` — cloud (any model via OpenRouter API)
- `ollama` — local/air-gapped (long-term government-run path)

## Environment variables

```
TPT__GOV__AI_LEVEL=none|advisory|assisted|automated
TPT__GOV__AI_PROVIDER=openrouter|ollama
TPT__GOV__AI_MODEL=anthropic/claude-sonnet-4-6          # for openrouter
TPT__GOV__AI_BASE_URL=http://localhost:11434             # for ollama
TPT__GOV__AI_API_KEY=sk-or-...                          # for openrouter
TPT__GOV__GATEWAY_LISTEN=0.0.0.0:8080
TPT__GOV__IDENTITY_LISTEN=0.0.0.0:8081
TPT__GOV__FEDERATION_LISTEN=0.0.0.0:7000
TPT__GOV__DEPT_ID=ird                                   # per dept node
DATABASE_URL=postgresql://user:pass@localhost:5432/db
```

## Development

```bash
# Start Phase 1 databases
docker compose -f docker/phase1.yml up -d

# Install TS dependencies
pnpm install

# Dev (all portals + packages)
pnpm dev

# Rust services
cargo build --workspace
cargo test --workspace

# Type check all TS
pnpm typecheck
```

## Phase 1 departments

| ID | Name | Service |
|----|------|---------|
| `ird` | Inland Revenue | `services/gov-dept-ird` |
| `winz` | Work and Income | `services/gov-dept-winz` |
| `moh` | Ministry of Health | `services/gov-dept-moh` |
| `dia` | Dept of Internal Affairs | `services/gov-dept-dia` |

Phase 2 and Phase 3 departments follow the same pattern. See `TODO.md` for the full list.

## Adding a new department

Follow the staged checklist in [CONTRIBUTING.md](./CONTRIBUTING.md#adding-a-department-stages-16). In short:

1. Add `services/gov-dept-<id>/` (based on `gov-dept-node`) and `services/gov-ingester-<id>/`, both in the root `Cargo.toml` workspace.
2. Add the dept's data schema to `packages/@tpt/gov-schema/src/departments/<id>.ts`.
3. Add relevant scopes to `ScopeSchema` in `packages/@tpt/gov-schema/src/identity.ts`.
4. Add the department's migrations, seed data and OPA policy (`policies/<id>.rego`).
5. Add the Postgres service and department container to `docker/phase1.yml`.
6. Add the department to the citizen and staff portals' registries.
