# Information Privacy Principles (Privacy Act 2020)

> Draft. Status labels are defined in [README.md](./README.md). Each principle applies
> to the agency that holds the information, so most rows are a combination of what
> the platform enforces and what the operating agency must do.

| IPP | Principle (summary) | Platform position | Status |
|---|---|---|---|
| 1 | Collect only information needed for a lawful purpose | Dept nodes store only the scopes modelled in their `DataBundle` schema. | Scaffold |
| 2 | Collect from the individual where practicable | Citizen data comes from department ingesters. The citizen sees each source in the audit trail. | Scaffold |
| 3 | Tell the individual what is collected and why | Consent screens list the scopes requested. No plain-language collection notice exists yet. | Planned |
| 4 | Collect lawfully and fairly | Ingesters read fixture or legacy transports only. No covert collection. | Scaffold |
| 5 | Keep information secure | Encryption in transit (mTLS, TLS), signed audit entries, consent-gated access, append-only action log. Storage encryption at rest and key management are agency responsibilities. | Scaffold |
| 6 | Access to own information | Citizen audit page (`/audit`). Full data export is not built. | Scaffold |
| 7 | Correction | Not built. | Planned |
| 8 | Check accuracy before use | Ingesters apply idempotent upserts; no validation workflow beyond schema checks. | Scaffold |
| 9 | Do not keep longer than necessary | No retention schedule or purge job for dept data. Grants have TTLs. | Planned |
| 10 | Use only for the purpose collected | OPA consent policies limit cross-department use to granted scopes. Purpose binding is planned (see capability ADR). | Scaffold |
| 11 | Disclose only where permitted | Disclosure happens only under a `DataGrantCredential`. Revocation is enforced at each dept node. | Implemented |
| 12 | Disclosure outside New Zealand | The optional OpenRouter AI provider is a cloud service and may process data outside NZ. The Ollama provider keeps data local. See [data residency](./data-residency-classification.md). | Scaffold |
| 13 | Unique identifiers | Identifiers are DIDs and department-local IDs. PII redaction covers NHI, IRD, passport and phone patterns before AI calls. | Scaffold |

## Notes for the operating agency

- The agency is the agency under the Act. The platform does not replace its privacy
  officer, its PIA process, or its breach response.
- IPP 3 (collection notice) and IPP 7 (correction) are the largest gaps. Both need
  product work before a pilot.
