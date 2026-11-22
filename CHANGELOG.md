# Changelog

All notable changes to this project are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). No versioned release has
been tagged yet.

## [Unreleased]

### Security

- Cross-department requests in every department service verify each consent
  grant's Ed25519 signature against the identity server's issuer key, and check
  every grant id against the identity server's revocation endpoint. All failures
  deny the request. OPA can only further restrict a decision.
- Citizen login requires the DID to be the fingerprint of the presented public key.
- Demo sign-in actions are gated on demo mode on the server, and the staff
  passphrase comparison is constant-time.
- Session secrets are required in production (`TPT__GOV__SESSION_SECRET`).
- The AI assistant reads only the departments and scopes granted to the session,
  redacts identifiers from context, and no longer returns upstream error details.

### Added

- `gov-consent` wiring in `gov-dept-node` and all department services.
- `TPT__GOV__ISSUER_PUBLIC_KEY` and `TPT__GOV__IDENTITY_API_KEY` in
  `docker/phase1.yml` and `.env.example`.
- Staff portal registry entries for HUD, MoJ and Police.
- Repository policy files: `SECURITY.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`,
  `NOTICE`, `.github/CODEOWNERS`, Dependabot configuration and issue templates.

### Fixed

- Removed references to packages that no longer exist (`services/gov-ai`,
  `services/gov-analytics`, `packages/adapters`) from the pnpm workspace and the
  citizen portal's TypeScript paths.

### Known issues

- 39 department services do not compile yet, because of placeholder `c.c` field
  accesses in their `routes.rs` and `sqlx` query macros that need a database.
