# gov-consent

Shared consent-grant verification for tpt-gov-nz department services.

Every department service must confirm two things before it serves data under a
consent grant:

1. The `DataGrantCredential` was genuinely signed by the identity server.
2. The grant has not been revoked (or never existed) since it was issued.

`gov-consent` implements both checks once, so the department services do not
each re-derive them. The checks **fail closed**: an expired grant, a bad
signature, an unreachable identity server, or an unknown grant id all deny the
request.

## Features

- **Signature verification** of `DataGrantCredential` against the identity
  server's Ed25519 public key.
- **Expiry checks** via `DataGrantCredential::is_valid`.
- **Revocation checks** against the identity server's batch
  `/v1/grants/status` endpoint, with a short-lived in-process cache.
- **Single and batch verification** (`verify_grant`, `verify_grants`).
- **Public key decoding** that accepts standard or URL-safe base64, padded or
  unpadded, because the browser wallet emits URL-safe base64.

## Installation

This crate is part of the tpt-gov-nz Cargo workspace and is consumed by path:

```toml
[dependencies]
gov-consent = { path = "../../crates/gov-consent" }
```

## Usage

Verify a single grant presented with a request:

```rust,ignore
use gov_consent::verify_grant;
use gov_identity_core::DataGrantCredential;

async fn authorise(grant: &DataGrantCredential) -> anyhow::Result<()> {
    // Checks expiry, signature, and revocation. Returns Err on any failure.
    verify_grant(grant).await?;
    Ok(())
}
```

Verify several grants in one request, using one batched revocation call:

```rust,ignore
use gov_consent::verify_grants;

verify_grants(&grants).await?;
```

Use the revocation client directly, for example to pre-check ids:

```rust,ignore
use gov_consent::RevocationClient;
use std::time::Duration;

let client = RevocationClient::new(
    "http://localhost:8081",
    "shared-api-key",
    Duration::from_secs(5),
    Duration::from_secs(30),
)?;
client.ensure_active(&[grant.id]).await?;
```

## Modules and public API

| Item | Purpose |
| --- | --- |
| `verify_grant` | Verify one grant: expiry, signature, revocation. |
| `verify_grants` | Verify many grants with a single batched revocation lookup. |
| `RevocationClient` | Batch grant-status client with a TTL cache. `new`, `from_env`, `ensure_active`. |
| `issuer_key_from_env` | Load the identity server's public key from the environment. |
| `decode_public_key` | Decode a 32-byte Ed25519 public key from base64. |

## Configuration

| Variable | Required | Default | Meaning |
| --- | --- | --- | --- |
| `TPT__GOV__ISSUER_PUBLIC_KEY` | yes | none | Identity server Ed25519 public key (standard or URL-safe base64). Startup fails if missing or empty. |
| `TPT__GOV__IDENTITY_SERVER_URL` | no | `http://localhost:8081` | Base URL of the identity server. |
| `TPT__GOV__IDENTITY_API_KEY` | yes | none | Shared key sent to the status endpoint. |
| `TPT__GOV__REVOCATION_TIMEOUT_SECS` | no | `5` | HTTP timeout for the status call. |
| `TPT__GOV__REVOCATION_CACHE_SECS` | no | `30` | How long a grant's active or revoked status is cached. |

See the repository's `.env.example` for a complete example.

## Security properties

- **Fail-closed.** An unreachable identity server, a malformed response, or a
  grant id the server reports as `unknown` all result in an error.
- **Bounded staleness.** A revoked grant can continue to pass for at most the
  cache TTL (`TPT__GOV__REVOCATION_CACHE_SECS`, default 30 seconds). Lower the
  TTL where revocation must take effect faster.
- **No silent acceptance.** `issuer_key_from_env` refuses to run without a key,
  so a department service cannot accidentally skip signature checks.

## Testing

```bash
cargo test -p gov-consent
```

Tests cover public key decoding (standard and URL-safe base64, invalid keys)
and that verification fails without the required environment configuration.

## Related crates

- [`gov-identity-core`](../gov-identity-core/README.md) defines the
  `DataGrantCredential` type and its signature check.
- [`gov-federation-node`](../gov-federation-node/README.md) uses this crate's
  revocation client to check grants on inbound QUIC federation requests.

## Contributing

See [CHANGELOG.md](CHANGELOG.md) for release history. Contributions follow the
repository guidelines in the root [README](../../README.md).

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE),
at your option.
