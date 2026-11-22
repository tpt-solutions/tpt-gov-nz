# gov-identity-core

Verifiable Credentials, DIDs, and identity logic for tpt-gov-nz.

This crate defines the identity types the platform shares: government DIDs,
signed Verifiable Credentials, and the `DataGrantCredential` that a citizen
issues to authorise one department to share data with another. It contains the
signing and verification logic for those types. It does not store anything and
does not perform network I/O.

The identity server ([`gov-identity-server`](../../services/gov-identity-server))
uses this crate to issue credentials. Department services use it to verify them.

## Features

- **Government DIDs.** `GovDid` values have the form
  `did:gov:nz:<base64url-encoded Ed25519 public key>`, so a DID is derived
  directly from its key.
- **DID documents.** `DidDocument` is built from a signing key.
- **Verifiable Credentials.** `VerifiableCredential` carries an issuer and
  subject DID, a credential type, validity times, free-form JSON claims, and an
  Ed25519 proof.
- **Data grant credentials.** `DataGrantCredential` authorises a requesting
  department to receive named scopes from a providing department, for a limited
  time, under a citizen's consent.
- **Issuance.** `CredentialIssuer` generates or restores an issuer key, then
  issues VCs and data grants.
- **Verification.** Checks for expiry and Ed25519 signatures. Revocation is out
  of scope here and is handled by [`gov-consent`](../gov-consent/README.md).

## Installation

```toml
[dependencies]
gov-identity-core = { path = "../../crates/gov-identity-core" }
```

## Usage

Issue a data grant from the identity server's key:

```rust,ignore
use gov_identity_core::{CredentialIssuer, GovDid};

let issuer = CredentialIssuer::from_secret_b64(&stored_secret)?;
let citizen = GovDid::parse("did:gov:nz:citizen001")?;

let grant = issuer.issue_data_grant(
    citizen,
    "winz",                           // requesting department
    "ird",                            // providing department
    vec!["ird:income".into()],        // scopes
    3600,                             // lifetime in seconds
);
```

Verify a grant signature with the issuer's public key:

```rust,ignore
grant.verify_signature(&issuer.verifying_key())?;
grant.is_valid()?; // checks expiry
```

Derive a DID from a public key:

```rust,ignore
use gov_identity_core::GovDid;

let did = GovDid::from_verifying_key(&verifying_key);
println!("{}", did.as_str());
```

## Modules and public API

| Module | Key items |
| --- | --- |
| `credential` | `VerifiableCredential`, `DataGrantCredential`, `CredentialIssuer` (`generate`, `from_secret_b64`, `secret_key_b64`, `public_key_b64`, `verifying_key`, `issue_vc`, `issue_data_grant`) |
| `did` | `GovDid` (`from_verifying_key`, `parse`, `as_str`), `DidDocument` |
| `error` | `IdentityError` |

## Credential model

A `DataGrantCredential` contains:

| Field | Meaning |
| --- | --- |
| `id` | Grant identifier, checked against the revocation endpoint. |
| `citizen_did` | The citizen who gave consent. |
| `requesting_dept_id` | The department that wants the data. |
| `providing_dept_id` | The department that holds the data. |
| `scopes` | The data scopes granted, for example `ird:income`. |
| `issued_at`, `expires_at` | Unix timestamps. |
| `signature` | Ed25519 signature by the issuer over the canonical fields. |

The signed payload excludes the signature field. Any change to a signed field
invalidates the credential.

## Security notes

- **Keep issuer keys secret.** `CredentialIssuer::secret_key_b64` exposes the
  private key. Only the identity server should hold it.
- **Short grant lifetimes.** Grants are time-limited. Choose the shortest
  lifetime that serves the use case.
- **Expiry is necessary, not sufficient.** A non-expired grant can still be
  revoked. Call [`gov-consent`](../gov-consent/README.md) to check revocation
  before serving data.

## Testing

```bash
cargo test -p gov-identity-core
```

## Related crates

- [`gov-consent`](../gov-consent/README.md) verifies grants in department services
  and checks revocation.
- [`gov-federation-core`](../gov-federation-core/README.md) references grant ids
  in federation envelopes.

## Contributing

See [CHANGELOG.md](CHANGELOG.md) for release history. Contributions follow the
repository guidelines in the root [README](../../README.md).

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE),
at your option.
