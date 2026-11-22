# Changelog

All notable changes to `gov-zk` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Pedersen-style commitments over the Ristretto group, with a second generator
  derived from a fixed domain-separation label.
- Schnorr and OR-proof building blocks (`ScalarProof`, `BitProof`) and a
  non-interactive `RangeProof` using the Fiat–Shamir transform with SHA-256.
- Age proofs: `prove_age` and `verify_age`, proving an age in the range 18 to
  150 from a committed birth year, without revealing it.
- Residency proofs: `prove_residency` and `verify_residency`, proving a
  residency flag of `1` without revealing other attributes.

### Notes

- These proofs are demo-grade and have not been audited. The `AgeProof` and
  `ResidencyProof` types are intended to be the integration boundary, so the
  range proof can later be replaced with a more succinct construction.
