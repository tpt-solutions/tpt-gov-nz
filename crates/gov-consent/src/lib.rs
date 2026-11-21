//! Shared consent-grant verification for department services.
//!
//! Every department service must (a) verify that a presented
//! [`gov_identity_core::DataGrantCredential`] was really signed by the identity
//! server, and (b) confirm the grant has not been revoked since it was issued.
//! This crate provides both pieces so the 40+ department services can share one
//! implementation instead of each re-deriving it.
//!
//! - [`issuer_key_from_env`] loads the identity server's Ed25519 public key from
//!   `TPT__GOV__ISSUER_PUBLIC_KEY` (standard or URL-safe base64).
//! - [`RevocationClient`] checks grant ids against the identity server's
//!   `/v1/grants/status` endpoint, with a short-lived cache.

mod issuer;
mod revocation;

pub use issuer::{decode_public_key, issuer_key_from_env};
pub use revocation::RevocationClient;
