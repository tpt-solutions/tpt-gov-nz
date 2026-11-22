# Governance

## Roles

- **Maintainers** decide what is built, merged and released. They implement all
  accepted changes themselves; see [CONTRIBUTING.md](./CONTRIBUTING.md).
- **Issue reporters** shape the roadmap by reporting bugs, requesting features and
  raising consent, privacy or security concerns.

Maintainers are listed in [`.github/CODEOWNERS`](./.github/CODEOWNERS).

## Decisions

- Day-to-day decisions are made by maintainers and recorded in the relevant issue.
- Changes to the shared contracts (`packages/@tpt/gov-schema`, the federation
  envelope, and the `DataGrantCredential` format) need an explicit written decision
  in an issue before implementation. Other departments depend on these.
- Security-relevant changes (consent, audit, authentication, federation) are
  reviewed against the principles in [CLAUDE.md](./CLAUDE.md) before release.

## Direction

The roadmap lives in [TODO.md](./TODO.md). The planned split into a core platform
repository and per-department repositories is described in its "Multi-Repo Platform
Split" section and is gated on the exit criteria listed there.

## Changes to this document

Governance changes are proposed and decided in a GitHub issue.
