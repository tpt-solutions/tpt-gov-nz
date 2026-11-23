# Threat model (draft)

This is a STRIDE-style starting point. It lists the trust boundaries, the main threats,
and the controls and gaps. Review it with each change to authentication, consent,
federation or the AI path.

## Trust boundaries

1. **Browser to portals** (citizen and staff). Untrusted input.
2. **Portal to gateway and to department nodes.** Internal, but must not trust caller
   identity from request bodies.
3. **Department node to identity server.** Verifies grants and checks revocation.
4. **Department node to OPA sidecar.** Optional policy decision point.
5. **Federation between nodes.** Signed envelopes over QUIC or HTTP.
6. **Platform to AI provider.** Cloud or local.

## Threats and controls

| # | Threat | STRIDE | Control | Gap / status |
|---|---|---|---|---|
| 1 | Forged or replayed consent grant | Spoofing, Tampering | Ed25519 signature check on every cross-department request (`enforce_signed_and_unrevoked`) | Implemented |
| 2 | Revoked grant still used | Elevation | Revocation check against identity server per request, cached 30 seconds | Scaffold: up to 30 s stale by design |
| 3 | Caller claims to be another department | Spoofing | Partial: grant checks apply whenever a `requesting_dept_id` is present | Open: a request that omits `requesting_dept_id` is treated as direct access, and the gateway does not yet bind caller identity to the JWT (see TODO.md) |
| 4 | Session forgery | Spoofing | HS256 session JWT: staff verified in middleware and in write actions; citizen verified on read | Implemented; citizen routes rely on per-page checks, not middleware |
| 5 | Credential guessing on staff login | Spoofing | Per-account lockout (5 failures, 15 min) | Scaffold: in-process only |
| 6 | Cross-site scripting | Tampering | Nonce-based CSP, React escaping | Scaffold: styles still allow `unsafe-inline` |
| 7 | Clickjacking | Tampering | `frame-ancestors 'none'`, `X-Frame-Options: DENY` | Implemented |
| 8 | Audit tampering | Repudiation, Tampering | Signed entries, hash chain, write rules on `actions_log` | Scaffold: no external anchoring |
| 9 | Personal data leaked to AI provider | Information disclosure | PII redaction on question and context | Scaffold: regex-based, not exhaustive |
| 10 | Excess data returned to AI | Information disclosure | AI route reads only granted scopes | Implemented |
| 11 | Secrets in repository | Information disclosure | `.env.example` with placeholders, gitleaks in CI | Implemented |
| 12 | Default secrets in production | Spoofing | Session secret required in production; identity API key minimum length | Implemented |
| 13 | Denial of service on public endpoints | Denial of service | Per-department rate limiting and circuit breaker at gateway | Scaffold: not measured under load in CI |
| 14 | Malicious policy change | Tampering | Policies in Git, CODEOWNERS, OPA check and test in CI | Scaffold: no policy simulation in CI yet |
| 15 | Compromised department ingester | Tampering | Sandboxing planned (`tpt-linux-sandbox`) | Planned |
| 16 | Unsigned federation messages | Spoofing | Ed25519 envelope signatures; replay protection partial | Open (see TODO.md federation item) |

## Review

Owner: the platform security lead (to be named). Review at each release and after
any incident. Record changes with the ADR process in `docs/adr/`.
