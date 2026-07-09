# tpt-gov-nz

Unified NZ government digital platform — open source, federated, AI-optional.

## Architecture

Hybrid monorepo: TypeScript (Turbo + pnpm) for portals and adapters; Rust (Cargo workspace) for systems-level services.

```
apps/            # Next.js 15 portals (portal-citizen, portal-staff, portal-policy, portal-open-data)
services/        # Long-running services
  gov-gateway/   # Rust/Axum API gateway
  gov-identity-server/  # DID + VC issuance
  gov-dept-node/ # Template each department runs
  gov-ai/        # TypeScript AI orchestrator (optional)
crates/          # Rust libraries
  gov-federation-core/  # Protocol types, Ed25519, envelopes
  gov-federation-node/  # QUIC peer runtime
  gov-identity-core/    # DID, Verifiable Credentials
packages/
  @tpt/gov-schema/      # Zod schemas — the single source of truth for types
  @tpt/gov-ai-client/   # Provider-agnostic AI (OpenRouter / Ollama)
  adapters/@tpt/adapter-*/  # One per department (IRD, WINZ, MOH, DIA, ...)
docker/          # Docker Compose for local dev
```

## Key design rules

1. **`DeptAdapter` interface is the contract** — adapters and native modules implement the same interface. The citizen portal never cares which mode a department is in.
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

| ID | Name | Adapter |
|----|------|---------|
| `ird` | Inland Revenue | `@tpt/adapter-ird` |
| `winz` | Work and Income | `@tpt/adapter-winz` |
| `moh` | Ministry of Health | `@tpt/adapter-moh` |
| `dia` | Dept of Internal Affairs | `@tpt/adapter-dia` |

Phase 2 will add: NZTA, ACC, MoE, MSD, MBIE, LINZ.

## Adding a new department

1. Create `packages/adapters/@tpt/adapter-<id>/` implementing `DeptAdapter`
2. Add the dept's data schema to `packages/@tpt/gov-schema/src/departments/<id>.ts`
3. Add relevant scopes to `ScopeSchema` in `packages/@tpt/gov-schema/src/identity.ts`
4. Add mock seed data to `docker/init/<id>.sql`
5. Add the Postgres service to `docker/phase1.yml`
6. Register the adapter in the citizen portal's dept registry
