# Privacy impact assessment template

Copy this template for each agency deployment or new department. Complete every
section. Leave a section blank only with a stated reason.

## 1. Project

- **Name and version:**
- **Agency and owner:**
- **Privacy officer:**
- **Date:**
- **Departments in scope:**

## 2. Purpose

- What problem does this solve, and for whom?
- Why is each data item needed for that purpose? (IPP 1)

## 3. Information flows

| Data item | Source | Destination | Legal basis or consent grant | Retention |
|---|---|---|---|---|
| | | | | |

Include cross-department flows, AI provider calls, and any transfer outside New Zealand.

## 4. Consent and notice

- Which `DataGrantCredential` scopes apply to each flow?
- What does the citizen see before consenting? (IPP 3)
- How can the citizen revoke, and how quickly does revocation take effect?

## 5. Security

- Access control (roles, OPA policies, staff authentication)
- Audit logging (what is logged, where it is kept, who can read it)
- Encryption in transit and at rest
- Key management, including the identity server issuer key and API keys

## 6. Risks

| Risk | Likelihood | Impact | Mitigation | Residual |
|---|---|---|---|---|
| | | | | |

Use [threat-model.md](./threat-model.md) as the starting list.

## 7. Individual rights

- Access (IPP 6), correction (IPP 7), and how each request is handled
- Process for queries from the citizen or the Privacy Commissioner

## 8. Breach response

Link the agency's breach runbook. See [breach-runbook.md](./breach-runbook.md).

## 9. Decision

- Proceed, proceed with conditions, or do not proceed
- Conditions and owners:
- Review date:
- Sign-off (privacy officer, system owner):
