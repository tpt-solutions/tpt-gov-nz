# Consent flow

This walks through one cross-department request, from grant to decision. Field names
match the Rust `DataGrantCredential` (snake_case JSON).

## 1. Grant

The citizen grants WINZ access to IRD income data. The identity server issues a signed
credential:

```json
{
  "id": "5b0c2f3e-6d1a-4b8e-9f2a-1c7d8e9f0a12",
  "citizen_did": "did:gov:nz:test-citizen-001",
  "requesting_dept_id": "winz",
  "providing_dept_id": "ird",
  "scopes": ["ird:income"],
  "issued_at": 1795000000,
  "expires_at": 1795003600,
  "signature": "<base64 Ed25519 signature over the canonical payload>"
}
```

`expires_at` is a Unix timestamp in seconds. Grants are short-lived. The citizen can
revoke one at `DELETE /v1/grants/{id}`.

## 2. Request

WINZ asks IRD for the data, through the gateway, and presents the grant:

```json
{
  "did": "did:gov:nz:test-citizen-001",
  "scopes": ["ird:income"],
  "requesting_dept_id": "winz",
  "consent_grants": [ { "...the grant above..." } ]
}
```

## 3. Check at the providing department

The IRD node runs these checks, in order. Any failure denies the request:

1. **Policy decision (optional).** If OPA is configured, `ird.consent` must allow.
   OPA can only further restrict.
2. **Signature.** Each covering grant's signature is verified against the identity
   server's issuer public key (`TPT__GOV__ISSUER_PUBLIC_KEY`).
3. **Scope and party.** Every requested scope is listed by a grant whose requesting
   department, providing department and citizen match, and which has not expired.
4. **Revocation.** Every presented grant id is checked against the identity server
   (`POST /v1/grants/status`). A revoked or unknown id denies the request.

The audit log records the decision on both sides, whatever the outcome.

## 4. Outcome

- Allowed: IRD returns the data for the requested scopes and writes an audit entry.
- Denied: IRD returns `403 Forbidden`. The audit entry lists the uncovered scopes.

## Try it locally

Start the identity server and an IRD node, issue a grant with `POST /v1/grants`, and
send the request above with `curl`. Then revoke the grant and send it again. The second
request must be denied.
