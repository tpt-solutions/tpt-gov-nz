# Data residency and classification

## Classification

The NZ Government Information Security Classification uses protective markings such
as IN-CONFIDENCE, SENSITIVE, RESTRICTED, CONFIDENTIAL, SECRET and TOP SECRET. The
agency sets the marking for each dataset. The platform does not assign markings.

Default markings for the demo and reference datasets:

| Dataset | Suggested marking | Notes |
|---|---|---|
| Fictional demo citizens (`docker/init`) | None (fictional) | Must never contain real personal information. |
| Citizen profile data from department nodes | SENSITIVE or higher, per the agency's assessment | Health, benefit and justice data may be higher. |
| Audit log | As the underlying data | Also protect against tampering. |
| Security and secrets configuration | RESTRICTED or higher | Never committed to the repository. |

Security agencies (GCSB, NZSIS, NZDF, Defence) are excluded from the public
repository. The multi-repo plan in `TODO.md` gives them private, air-gapped builds.

## Residency

| Component | Default location | Data sent off-shore? |
|---|---|---|
| Department nodes and databases | Agency-chosen NZ hosting | No, if deployed in NZ |
| Identity server | Agency-chosen NZ hosting | No |
| AI, OpenRouter provider | Provider-chosen infrastructure | **Yes, if enabled.** Redacted question and context chunks are sent. |
| AI, Ollama provider | Agency hosts it | No |

Recommendation for NZ government use: run the Ollama provider in NZ, and keep
OpenRouter disabled (`TPT__GOV__AI_LEVEL=none`) for any environment that holds real
citizen data. Confirm this with the agency's privacy officer under IPP 12.

## Backups and disposal

No backup or disposal job exists yet. The agency must define retention per dataset
(see IPP 9 in [privacy-act-ipps.md](./privacy-act-ipps.md)).
