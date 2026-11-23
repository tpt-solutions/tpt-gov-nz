# Algorithmic Charter statement (draft)

The Algorithmic Charter for Aotearoa New Zealand is a government commitment to
transparent, ethical and inclusive use of algorithms in the public sector. An agency
that adopts the charter publishes a statement of how it meets each commitment. This
document is a draft for the operating agency to adapt.

## What the platform does with algorithms

- **AI assistance** for citizens and case workers, provided through a provider-agnostic
  client. Providers are OpenRouter (cloud) or Ollama (local). The AI level setting
  (`none`, `advisory`, `assisted`, `automated`) controls what the AI may do.
- **Rules-based eligibility and consent decisions** in OPA policies, which are readable
  and testable.
- **Policy simulation** in `apps/portal-policy`, which compares scenarios.

## Position against the charter's commitments

| Commitment | Platform position | Status |
|---|---|---|
| Transparency: say when an algorithm is used | The AI assistant is labelled. Consent and eligibility rules are visible in the Rego source. | Scaffold |
| Explainability: explain decisions in plain language | AI answers are not yet linked to the data they used. Rules are not yet explained in plain language to citizens. | Planned |
| Human oversight: a person decides | `advisory` and `assisted` levels keep a person in the decision. `automated` is not implemented for benefit or entitlement decisions. | Scaffold |
| Fairness and bias testing | No bias testing for the AI or the policies. | Planned |
| Data quality and privacy | PII redaction before AI calls. Context chunks are redacted too. | Scaffold |
| Accountability: named owner, review process | Governance documents exist. Each agency must name its owner. | Scaffold |

## Commitments the agency must make

- Name an accountable owner for each algorithmic system.
- Publish this statement with the agency's version and the date of its last review.
- Run a bias and fairness assessment before using `assisted` or `automated` levels on
  any decision about people.
