# 0001. Capability token format: extend `DataGrantCredential`

- **Status:** Accepted
- **Date:** 2026-11-23
- **Decides:** the first two items under "Capability-based security" in `TODO.md`

## Context

Department nodes must accept only capabilities, with no ambient authority. A
capability names a resource, the rights granted, and caveats such as expiry,
purpose, audience and maximum uses. It can be delegated only by attenuation, and
the verifier must check the whole chain.

Three existing candidates were compared:

- **Biscuit**: public-key verification, Datalog-based caveats, offline attenuation.
- **UCAN**: DID-based capabilities carried in JWT-like tokens, with delegation chains.
- **Macaroons**: HMAC-chained caveats, attenuation by any holder.

The platform already has `DataGrantCredential` (`crates/gov-identity-core/src/credential.rs`).
It is an Ed25519-signed credential issued by the identity server. Every department
node verifies it through `gov-consent`, and the verification path is tested.

## Decision

Keep `DataGrantCredential` as the capability format and evolve it in place:

1. Add holder binding: the grant carries the holder's public key, and a use of the
   grant must be signed by that key (proof of possession).
2. Add caveats as typed fields: `resource` (dept and dataset), `rights`, `purpose`,
   `audience` (the requesting department), `max_uses`, and `expires_at`.
3. Delegation by attenuation: a derived grant is a new credential that references
   its parent by id and hash. It is signed by the holder of the parent and may only
   narrow the parent's resource, rights, caveats and expiry. Verifiers walk the
   chain back to an identity-server-signed root.
4. Revocation stays a server-side list (already implemented as the grant-status
   endpoint). Short TTLs bound how long a leaked grant is useful.

## Reasons

- **No new trust format.** Ed25519 signatures, the issuer key distribution, and the
  revocation client already exist and are tested. Adopting Biscuit or UCAN would
  add a second verification path in every department node.
- **No shared secrets.** Macaroons verify with the minting key, so every verifier
  would need the root secret. That conflicts with federated verification.
- **Procurement and review.** Ed25519 is a standard, well-reviewed primitive, which
  simplifies review against NZISM cryptographic guidance.
- **Attenuation is still achievable.** A signed parent-to-child chain gives the
  narrowing-only property without a new library.

## Consequences

- We implement attenuation ourselves, so the chain-verification code needs its own
  review and conformance tests (forged, widened, wrong-holder, expired, revoked).
- Biscuit's Datalog policies are richer than our typed caveats. We accept that
  limit. If a department needs expressive policy, OPA remains the place to express it.
- Existing grants stay valid during migration because the new fields are optional
  and default to the current behaviour.

## Alternatives considered

| Option | Why not chosen |
|---|---|
| Biscuit | New dependency and trust format in every node; library maturity for our use is untested. |
| UCAN | Larger token and resolver surface; DID-resolution dependency for every check. |
| Macaroons | Verification needs the minting secret; unsuitable for federation. |

## Follow-up

The "Evolve `DataGrantCredential`" item in `TODO.md` is the implementation of this
decision. The "Single shared verification path" item is only partly met: department
services share the `gov-consent` issuer-key loader and `RevocationClient`, through
the per-service `enforce_signed_and_unrevoked` helper. They do not yet call
`gov_consent::verify_grant` (the shared, single-call function), so that is the next
step for consolidating the path.
