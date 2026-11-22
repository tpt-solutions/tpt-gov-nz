# gov-zk

Zero-knowledge age and residency proofs for tpt-gov-nz.

`gov-zk` lets a citizen prove a fact about themselves without revealing the
underlying value:

- **Age.** Prove they are an adult, meaning the age derived from their birth
  year is in the range 18 to 150, without disclosing their date of birth.
- **Residency.** Prove their residency flag is `1` (a New Zealand resident),
  without disclosing anything else.

A verifier learns only that the committed value lies in the claimed range.

> **Status: demo-grade, not audited.** The crate's own documentation states
> that these proofs are sound under the discrete-log assumption and the
> Fiat–Shamir heuristic, but they have not been independently reviewed. Do not
> rely on them for production eligibility decisions until they have been
> audited.

## How it works

- Values are committed as Pedersen-style commitments `C = value·G + r·H` over
  the Ristretto prime-order group, using `curve25519-dalek`.
- The second generator `H` is derived by hashing a fixed domain-separation label,
  so no party knows its discrete log relative to `G`.
- Range checks use a Schnorr / OR-proof construction on bit commitments
  (`BitProof`, `RangeProof`).
- Proofs are made non-interactive with the Fiat–Shamir transform, using SHA-256.
- There is no trusted setup.

For production, the crate's documentation anticipates swapping the range proof
for a more succinct system such as Bulletproofs or a SNARK. The `AgeProof` and
`ResidencyProof` types are the integration boundary, so callers should not
depend on the internal proof structure.

## Installation

```toml
[dependencies]
gov-zk = { path = "../../crates/gov-zk" }
```

## Usage

Prove adulthood from a birth year, keeping the blinding factor secret:

```rust,ignore
use curve25519_dalek::scalar::Scalar;
use gov_zk::{prove_age, verify_age};
use rand::rngs::OsRng;

let blinding = Scalar::random(&mut OsRng);
let proof = prove_age(1990, blinding, 2026)?; // birth year, blinding, current year
assert!(verify_age(&proof));
```

Prove residency from a status flag:

```rust,ignore
use gov_zk::{prove_residency, verify_residency};

let proof = prove_residency(1, blinding);
assert!(verify_residency(&proof));
```

The verifier receives the proof object and needs no secrets. Verification
returns `bool`: `true` means the claim holds, and `false` means the proof is
invalid or was tampered with.

## Public API

| Item | Purpose |
| --- | --- |
| `prove_age(birth_year, blinding, current_year)` | Produce an `AgeProof`. Errors if the birth year is in the future. |
| `verify_age(&AgeProof)` | Return `true` if the age proof verifies. |
| `prove_residency(status, blinding)` | Produce a `ResidencyProof` for a status of `1`. |
| `verify_residency(&ResidencyProof)` | Return `true` if the residency proof verifies. |
| `AgeProof`, `ResidencyProof` | Proof types and the integration boundary. |
| `ScalarProof`, `BitProof`, `RangeProof` | Building blocks, exposed for inspection and testing. |

## Security notes

- **Keep blinding factors secret.** Anyone who learns the blinding factor and
  the committed value can link the proof to the citizen.
- **Use fresh randomness.** Generate each blinding factor from a
  cryptographically secure random source, for example `OsRng`.
- **Check `current_year`.** Age is computed from the year supplied by the caller.
  Verifiers must supply or check the same year the prover used.
- **Not audited.** See the status note above.

## Testing

```bash
cargo test -p gov-zk
```

Tests cover round trips for age and residency proofs, rejection of tampered
proofs, and binding of range proofs to their commitments.

## Related crates

- [`gov-identity-core`](../gov-identity-core/README.md) handles the verifiable
  credentials that these proofs could complement.

## Contributing

See [CHANGELOG.md](CHANGELOG.md) for release history. Contributions follow the
repository guidelines in the root [README](../../README.md). Cryptographic
changes need a careful review and should come with new tests.

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE),
at your option.
