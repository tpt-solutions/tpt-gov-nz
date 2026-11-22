# gov-federation-core

Core types and protocol definitions for the tpt-gov-nz federation protocol.

This crate contains the transport-independent building blocks that every
federation participant shares: signed message envelopes, signed audit log
entries, node identities, and the error type. It deliberately has no network
code. Use [`gov-federation-node`](../gov-federation-node/README.md) for the
runtime that sends and receives these messages.

## Features

- **Signed envelopes.** `FederationEnvelope` carries a message id, a message
  type, the sending and receiving department ids, a timestamp, the consent
  grant ids that authorise the exchange, an opaque payload string, and an
  Ed25519 signature over a canonical JSON form.
- **Message types.** `FederationMessageType` is one of `DATA_REQUEST`,
  `DATA_RESPONSE`, `DATA_DENIED`, or `AUDIT_ACK`.
- **Signed audit entries.** `AuditLogEntry` records an exchange and can be
  signed and verified, so both sides of an exchange hold a verifiable record.
- **Node identities.** `NodeKeypair` generates or restores an Ed25519 keypair
  for a department, and `DeptIdentity` describes a department.
- **Typed errors.** `FederationError` covers invalid consent, signature
  failures, unauthorised scopes, expired grants, unknown departments, and
  serialisation, crypto, and transport failures.

## Installation

```toml
[dependencies]
gov-federation-core = { path = "../../crates/gov-federation-core" }
```

## Usage

Create a keypair for a department and build a signed `DATA_REQUEST`:

```rust,ignore
use gov_federation_core::{FederationEnvelope, NodeKeypair};
use uuid::Uuid;

let keypair = NodeKeypair::generate("winz");
println!("public key: {}", keypair.public_key_b64());

let grant_ids = vec![Uuid::new_v4()];
let mut envelope = FederationEnvelope::new_request(
    "winz",
    "ird",
    grant_ids,
    "<opaque payload>".to_string(), // produced by the caller
    String::new(),                  // filled in by sign()
);
// envelope.sign(&signing_key);
// envelope.verify(&sender_verifying_key)?;
```

Restore a keypair from a stored secret:

```rust,ignore
let keypair = NodeKeypair::from_secret_b64(&stored_secret_b64)?;
```

## Modules and public API

| Module | Key items |
| --- | --- |
| `envelope` | `FederationEnvelope`, `FederationMessageType`, `new_request`, `sign`, `verify` |
| `audit` | `AuditLogEntry`, `new`, `sign`, `verify` |
| `identity` | `NodeKeypair` (`generate`, `from_secret_b64`, `from_secret_b64_with_dept_id`, `public_key_b64`, `secret_key_b64`), `DeptIdentity` |
| `error` | `FederationError` |

## Wire format

Envelopes serialise as JSON with camelCase field names, for example
`messageId`, `messageType`, `fromDeptId`, `toDeptId`, `timestampMs`,
`consentGrantIds`, `payloadEncrypted`, and `signature`. Message types are
serialised in `SCREAMING_SNAKE_CASE`.

The signature covers a canonical JSON object containing every field except
`signature`, hashed with SHA-256 and signed with Ed25519. Any change to a signed
field invalidates the envelope.

## Security notes

- The `payload_encrypted` field is an opaque string. This crate **does not**
  encrypt or decrypt payloads. Encryption is the caller's responsibility.
- Signature verification is necessary but not sufficient. Receivers must also
  check consent grants, timestamps, and replay protection. The federation node
  does this.
- Secret keys are returned as base64 strings by `secret_key_b64`. Store them
  only in a secrets manager, never in source control.

## Testing

```bash
cargo test -p gov-federation-core
```

## Related crates

- [`gov-federation-node`](../gov-federation-node/README.md) is the QUIC and HTTP
  runtime that transports these envelopes.
- [`gov-identity-core`](../gov-identity-core/README.md) provides the consent
  credentials referenced by `consent_grant_ids`.
- [`gov-consent`](../gov-consent/README.md) verifies those grants.

## Contributing

See [CHANGELOG.md](CHANGELOG.md) for release history. Contributions follow the
repository guidelines in the root [README](../../README.md).

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE),
at your option.
