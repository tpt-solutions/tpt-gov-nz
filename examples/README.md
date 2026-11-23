# Examples

Small, runnable references for people adding a department or building on the consent
model. These are references, not production code. Start with the consent flow, then
read the policy example.

| Path | What it shows |
|---|---|
| [consent-flow.md](./consent-flow.md) | How a citizen grants a department access, and how a cross-department request is checked. Includes the request and grant shapes. |
| [minimal-dept/policy.rego](./minimal-dept/policy.rego) | A complete consent policy for a fictional department, with tests in `minimal-dept/policy_test.rego`. Run with `opa test examples`. |

For a real department, follow [CONTRIBUTING.md](../CONTRIBUTING.md#adding-a-department-stages-16),
and copy the IRD policy in `policies/ird.rego` and its tests rather than writing from
scratch.

Not yet provided: an "agency in a box" Docker Compose file. It is tracked in
`TODO.md`.
