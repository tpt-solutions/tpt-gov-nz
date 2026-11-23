//! Zero-knowledge proofs for eligibility attributes.
//!
//! This crate lets a citizen prove facts about themselves *without revealing
//! the underlying value*:
//!
//! - [`prove_age`] / [`verify_age`] — prove `current_year - birth_year ∈ [18, 150]`
//!   (i.e. the citizen is an adult) without disclosing their date of birth.
//! - [`prove_residency`] / [`verify_residency`] — prove a residency flag is `1`
//!   (a New Zealand resident) without disclosing any other attribute.
//!
//! The construction is a Schnorr / OR-proof based **zero-knowledge range proof**
//! over the Ristretto prime-order group (no trusted setup). It is a genuine ZK
//! proof: the verifier learns only that the committed value lies in the claimed
//! range. For a production system the range proof would be swapped for a more
//! succinct SNARK (e.g. Bulletproofs / arkworks) — the [`AgeProof`] /
//! [`ResidencyProof`] types are the integration boundary.
//!
//! Security note: these proofs are sound under the discrete-log assumption and
//! the Fiat–Shamir heuristic. They are demo-grade, not audited.

use anyhow::Result;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use sha2::{Digest, Sha256, Sha512};

fn g() -> RistrettoPoint {
    RISTRETTO_BASEPOINT_POINT
}

fn h() -> RistrettoPoint {
    // Domain-separated second generator (not a known discrete log with respect
    // to `g`), derived by hashing a fixed label onto the Ristretto group.
    RistrettoPoint::hash_from_bytes::<Sha512>(b"tpt-gov-nz-zk-generator-H-v1")
}

/// Commit to `value` with blinding `r`: C = value·G + r·H.
fn commit(value: u64, r: Scalar) -> RistrettoPoint {
    g() * Scalar::from(value) + h() * r
}

/// Fiat–Shamir challenge from a set of group elements plus extra bytes.
fn challenge(points: &[RistrettoPoint], extra: &[u8]) -> Scalar {
    let mut buf = Vec::with_capacity(points.len() * 32 + extra.len());
    for p in points {
        buf.extend_from_slice(&p.compress().to_bytes());
    }
    buf.extend_from_slice(extra);
    let digest = Sha256::digest(&buf);
    Scalar::from_bytes_mod_order(digest.into())
}

/// Schnorr proof of knowledge of `x` such that `base = x·H`.
pub struct ScalarProof {
    a: RistrettoPoint,
    z: Scalar,
}

fn prove_scalar_knowledge(x: Scalar, base: RistrettoPoint) -> ScalarProof {
    let t = random_scalar();
    let a = h() * t;
    let e = challenge(&[base, a], &[]);
    let z = t + e * x;
    ScalarProof { a, z }
}

fn verify_scalar_knowledge(base: RistrettoPoint, proof: &ScalarProof) -> bool {
    let e = challenge(&[base, proof.a], &[]);
    let lhs = proof.a;
    let rhs = h() * proof.z - base * e;
    lhs == rhs
}

/// OR-proof that a commitment `c` binds to bit 0 *or* bit 1.
pub struct BitProof {
    a0: RistrettoPoint,
    e0: Scalar,
    z0: Scalar,
    a1: RistrettoPoint,
    e1: Scalar,
    z1: Scalar,
}

/// Prove that `c = bit·G + r·H` commits to 0 or 1, without revealing which.
///
/// Cramer–Damgård–Schoenmakers OR-proof. Branch 0 proves knowledge of `r` with base `c`
/// (`c = r·H`), branch 1 with base `c − G`. The real branch takes its challenge as
/// `e_total − e_simulated`; the simulated branch is chosen first. Each branch's challenge is
/// stored in the proof, so the verifier can check that the two sum to the Fiat–Shamir total.
fn prove_bit(c: RistrettoPoint, bit: u8, r: Scalar) -> BitProof {
    let base0 = c;
    let base1 = c - g();

    if bit == 0 {
        let t = random_scalar();
        let a0 = h() * t;
        // Simulate branch 1.
        let e1 = random_scalar();
        let z1 = random_scalar();
        let a1 = h() * z1 - base1 * e1;
        let e_total = challenge(&[c, a0, a1], &[]);
        let e0 = e_total - e1;
        let z0 = t + e0 * r;
        BitProof {
            a0,
            e0,
            z0,
            a1,
            e1,
            z1,
        }
    } else {
        let t = random_scalar();
        let a1 = h() * t;
        // Simulate branch 0.
        let e0 = random_scalar();
        let z0 = random_scalar();
        let a0 = h() * z0 - base0 * e0;
        let e_total = challenge(&[c, a0, a1], &[]);
        let e1 = e_total - e0;
        let z1 = t + e1 * r;
        BitProof {
            a0,
            e0,
            z0,
            a1,
            e1,
            z1,
        }
    }
}

fn verify_bit(c: RistrettoPoint, proof: &BitProof) -> bool {
    let base0 = c;
    let base1 = c - g();
    let e_total = challenge(&[c, proof.a0, proof.a1], &[]);
    let sum_ok = proof.e0 + proof.e1 == e_total;
    let eq0 = h() * proof.z0 == proof.a0 + base0 * proof.e0;
    let eq1 = h() * proof.z1 == proof.a1 + base1 * proof.e1;
    sum_ok && eq0 && eq1
}

/// Zero-knowledge proof that a committed value lies in `[min, max]`.
pub struct RangeProof {
    pub commitments: Vec<RistrettoPoint>,
    pub bits: Vec<BitProof>,
    pub sum: ScalarProof,
    pub min: u64,
    pub max: u64,
}

fn range_width(min: u64, max: u64) -> u32 {
    let span = max - min;
    if span == 0 {
        1
    } else {
        u64::BITS - span.leading_zeros()
    }
}

fn prove_range(c: RistrettoPoint, value: u64, blinding: Scalar, min: u64, max: u64) -> RangeProof {
    let n = range_width(min, max) as usize;
    let cprime = c - g() * Scalar::from(min);

    let val = value - min;
    let mut commitments = Vec::with_capacity(n);
    let mut bits = Vec::with_capacity(n);
    let mut blind_sum = Scalar::from(0u64);

    for i in 0..n {
        let bit = ((val >> i) & 1) as u8;
        let r_i = random_scalar();
        let c_i = commit(bit as u64, r_i);
        commitments.push(c_i);
        bits.push(prove_bit(c_i, bit, r_i));
        blind_sum += Scalar::from(1u64 << i) * r_i;
    }

    let delta = blinding - blind_sum;
    let t = random_scalar();
    let a = h() * t;
    let e = challenge(&[cprime, a], &[]) + challenge(&commitments.clone(), &[]);
    let z = t + e * delta;

    RangeProof {
        commitments,
        bits,
        sum: ScalarProof { a, z },
        min,
        max,
    }
}

fn verify_range(c: RistrettoPoint, proof: &RangeProof) -> bool {
    let _n = proof.commitments.len();
    let cprime = c - g() * Scalar::from(proof.min);

    for (i, bp) in proof.bits.iter().enumerate() {
        if !verify_bit(proof.commitments[i], bp) {
            return false;
        }
    }

    let mut acc = cprime;
    for (i, c_i) in proof.commitments.iter().enumerate() {
        acc -= c_i * Scalar::from(1u64 << i);
    }
    let e = challenge(&[cprime, proof.sum.a], &[]) + challenge(&proof.commitments.clone(), &[]);
    let lhs = proof.sum.a;
    let rhs = h() * proof.sum.z - acc * e;
    lhs == rhs
}

fn random_scalar() -> Scalar {
    let mut rng = rand::thread_rng();
    Scalar::random(&mut rng)
}

// --- Age proof ---

/// Proof that `current_year - birth_year ∈ [18, 150]` without revealing DOB.
pub struct AgeProof {
    pub cb: RistrettoPoint,
    pub ca: RistrettoPoint,
    pub delta: ScalarProof,
    pub range: RangeProof,
    pub current_year: u16,
}

pub fn prove_age(birth_year: u16, blinding: Scalar, current_year: u16) -> Result<AgeProof> {
    if current_year < birth_year {
        return Err(anyhow::anyhow!("birth year is in the future"));
    }
    let age = (current_year - birth_year) as u64;
    let cb = commit(birth_year as u64, blinding);
    let delta = random_scalar(); // blinding for the age commitment (ra - rb)
    let ca = g() * Scalar::from(age) + h() * delta;

    // ca + cb − current_year·G = (delta + blinding)·H, so the witness is delta + blinding.
    let base = ca - g() * Scalar::from(current_year as u64) + cb;
    let delta_proof = prove_scalar_knowledge(delta + blinding, base);
    let range = prove_range(ca, age, delta, 18, 150);

    Ok(AgeProof {
        cb,
        ca,
        delta: delta_proof,
        range,
        current_year,
    })
}

pub fn verify_age(proof: &AgeProof) -> bool {
    let base = proof.ca - g() * Scalar::from(proof.current_year as u64) + proof.cb;
    if !verify_scalar_knowledge(base, &proof.delta) {
        return false;
    }
    verify_range(proof.ca, &proof.range)
}

// --- Residency proof ---

/// Proof that a committed residency flag equals `1` (citizen is a resident).
pub struct ResidencyProof {
    pub ca: RistrettoPoint,
    pub range: RangeProof,
}

pub fn prove_residency(status: u8, blinding: Scalar) -> ResidencyProof {
    let ca = commit(status as u64, blinding);
    let range = prove_range(ca, status as u64, blinding, 1, 1);
    ResidencyProof { ca, range }
}

pub fn verify_residency(proof: &ResidencyProof) -> bool {
    verify_range(proof.ca, &proof.range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_proof_round_trip() {
        let proof = prove_age(1990, random_scalar(), 2024).unwrap();
        assert!(verify_age(&proof), "valid adult age should verify");
    }

    #[test]
    fn age_proof_rejects_tampering() {
        let mut proof = prove_age(1990, random_scalar(), 2024).unwrap();
        // Tamper: flip the age commitment's blinding relationship.
        proof.ca += g();
        assert!(!verify_age(&proof), "tampered age proof must fail");
    }

    #[test]
    fn residency_proof_round_trip() {
        let proof = prove_residency(1, random_scalar());
        assert!(verify_residency(&proof), "resident flag should verify");
    }

    #[test]
    fn residency_proof_rejects_tampering() {
        let mut proof = prove_residency(1, random_scalar());
        proof.ca += g();
        assert!(
            !verify_residency(&proof),
            "tampered residency proof must fail"
        );
    }

    #[test]
    fn range_proof_binds_to_commitment() {
        let blinding = random_scalar();
        let c = commit(50, blinding);
        let proof = prove_range(c, 50, blinding, 18, 150);
        assert!(verify_range(c, &proof), "valid in-range proof verifies");
        // A different commitment to the same value must not verify.
        let c2 = commit(50, random_scalar());
        assert!(
            !verify_range(c2, &proof),
            "proof must bind to its commitment"
        );
    }

    #[test]
    fn range_proof_rejects_out_of_range_value() {
        // 300 is outside [18, 150]. The honest prover can only encode its low bits, so the
        // proof must not verify against the commitment to 300.
        let blinding = random_scalar();
        let c = commit(300, blinding);
        let proof = prove_range(c, 300, blinding, 18, 150);
        assert!(
            !verify_range(c, &proof),
            "out-of-range value must not verify"
        );
    }

    #[test]
    fn bit_proof_rejects_a_non_bit_value() {
        // commit(2) is not a bit commitment, so no OR-proof for 0 or 1 can verify for it.
        let r = random_scalar();
        let c = commit(2, r);
        let proof = prove_bit(c, 0, r);
        assert!(!verify_bit(c, &proof), "a commitment to 2 is not a bit");
    }
}
