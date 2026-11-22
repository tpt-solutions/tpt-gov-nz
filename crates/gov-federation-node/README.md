# gov-federation-node

QUIC-based (and HTTP mock) federation protocol node for tpt-gov-nz.

A federation node is the runtime each department runs to exchange signed
`FederationEnvelope` messages with its peers. It handles inbound requests,
outbound sends, peer registration, enrolment, and the protections that stop
replayed or flooded traffic. The message types and signatures come from
[`gov-federation-core`](../gov-federation-core/README.md).

## Features

- **Two transports.**
  - An HTTP transport, the default. It is the Phase 1 local-development mock.
  - A QUIC transport behind the `quic` Cargo feature. It uses `quinn` with
    self-signed certificates and the ALPN identifier `tpt-gov-nz/1`.
- **Peer configuration** from environment variables, or a compact string format.
- **Replay protection.** A bounded window of recently seen message ids
  (10,000 entries).
- **Timestamp skew checks** (QUIC inbound path). Messages more than 60 seconds
  from the local clock are rejected.
- **Rate limiting.** A per-peer window limiter. The defaults are 60 requests per
  minute and up to 100 tracked peers.
- **Signature verification** of inbound envelopes against the registered
  sender's public key.
- **Consent gating** (QUIC inbound path). Consent grant ids on inbound requests
  are checked for revocation through [`gov-consent`](../gov-consent/README.md).
- **Node registry.** A file-backed registry of peers, enrolment tokens, and
  last-seen times.

## Installation

```toml
[dependencies]
gov-federation-node = { path = "../../crates/gov-federation-node" }

# To enable QUIC transport:
# gov-federation-node = { path = "../../crates/gov-federation-node", features = ["quic"] }
```

## Usage

Build a node from the environment and start its listener:

```rust,ignore
use gov_federation_node::{FederationNode, FederationNodeConfig};

let config = FederationNodeConfig::from_env()?;
let node = FederationNode::new(config);
node.start().await?;
```

Install a handler for inbound envelopes:

```rust,ignore
use gov_federation_node::{EnvelopeHandler, FederationNode};

let node = FederationNode::new(config).with_handler(handler);
```

Enable QUIC with `start_quic` and `send_quic` when the `quic` feature is on.

## Configuration

| Variable | Required | Default | Meaning |
| --- | --- | --- | --- |
| `TPT__GOV__DEPT_ID` | yes | none | This department's id, for example `ird`. |
| `TPT__GOV__FEDERATION_LISTEN` | no | `0.0.0.0:7000` | Socket address to listen on. |
| `TPT__GOV__FEDERATION_PEERS` | no | empty | Comma-separated peer list. |

Peer entries use the format `dept_id@host:port#base64_public_key`:

```text
TPT__GOV__FEDERATION_PEERS=ird@127.0.0.1:7001#<key>,winz@127.0.0.1:7002#<key>
```

Entries without `@` or `#`, or with an invalid socket address, are rejected at
startup.

## Protocol constants

| Constant | Value | Meaning |
| --- | --- | --- |
| `ENVELOPE_PATH` | `/federation/v1/envelope` | HTTP route for inbound envelopes. |
| `QUIC_ALPN` | `tpt-gov-nz/1` | QUIC application protocol identifier. |
| `MAX_TIMESTAMP_SKEW_SECS` | `60` | Maximum accepted clock skew. |
| `REPLAY_WINDOW_SIZE` | `10_000` | Message ids remembered for replay detection. |
| `DEFAULT_FEDERATION_RATE_LIMIT_RPM` | `60` | Requests per minute per peer. |
| `DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS` | `100` | Maximum peers tracked by the limiter. |

## Feature flags

| Feature | Default | Effect |
| --- | --- | --- |
| `quic` | off | Adds the `quic` module, `QuicTransport`, `serve_quic`, and the QUIC endpoint helpers. Pulls in `rcgen` and `bytes`. |

## Security notes

- **Authenticate every envelope.** Inbound messages are checked for signature,
  replay, and rate limit before any handler runs. On the QUIC path, timestamp
  skew and consent grant revocation are also checked. Do not bypass these
  checks in custom handlers.
- **Peer keys are configuration.** A peer's public key comes from its
  configuration or enrolment. Verify key distribution out of band.
- **The HTTP transport is a development mock.** It does not provide transport
  encryption or client authentication. Use the QUIC transport for any deployment
  that is not local development. This crate does not integrate
  [`gov-mtls`](../gov-mtls/README.md); mTLS is a separate concern for the
  HTTP-based services.
- **QUIC certificates are self-signed.** Trust comes from the pinned peer
  certificate and the envelope signatures, not from a public CA.

## Testing

```bash
cargo test -p gov-federation-node
cargo test -p gov-federation-node --features quic
```

## Related crates

- [`gov-federation-core`](../gov-federation-core/README.md) defines envelopes,
  identities, and errors.
- [`gov-consent`](../gov-consent/README.md) verifies consent grants on requests.

## Contributing

See [CHANGELOG.md](CHANGELOG.md) for release history. Contributions follow the
repository guidelines in the root [README](../../README.md).

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE),
at your option.
