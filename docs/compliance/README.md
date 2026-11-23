# Compliance pack (draft)

> **Status: draft, not a compliance claim.** These documents map what the platform
> does today against New Zealand requirements. They do not certify anything. Each
> control is labelled, and every label must be re-checked before any pilot or
> production use. Legal and security reviews are still required (see the external
> audit item in `TODO.md`).

## Status labels

| Label | Meaning |
|---|---|
| **Implemented** | Code exists, is tested, and is wired into the default stack. |
| **Scaffold** | Code or config exists, but it is partial, not wired by default, or not tested end to end. |
| **Planned** | Not built. Listed so the gap is visible. |
| **Not applicable** | Out of scope for this codebase. The owning agency handles it. |

## Contents

- [Information Privacy Principles (Privacy Act 2020)](./privacy-act-ipps.md)
- [Privacy impact assessment template](./pia-template.md)
- [Notifiable privacy breach runbook](./breach-runbook.md)
- [NZISM control mapping](./nzism-mapping.md)
- [Māori data sovereignty (Te Mana Raraunga and CARE)](./maori-data-sovereignty.md)
- [Algorithmic Charter statement](./algorithmic-charter.md)
- [Data residency and classification](./data-residency-classification.md)
- [Threat model](./threat-model.md)
- [SBOM and supply chain](./sbom.md)

## Scope

The platform is open-source software. Each department runs its own instance and is
the controller of its data. This pack covers the software and its default
configuration. Operating controls (premises, personnel, hosting contracts) belong to
each agency.
