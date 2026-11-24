pub mod capability;
pub mod credential;
pub mod did;
pub mod error;

pub use capability::{
    CapabilityChain, CapabilityError, CapabilityLink, EffectiveCapability, HolderKey, verify_chain,
};
pub use credential::{CredentialIssuer, DataGrantCredential, VerifiableCredential};
pub use did::{DidDocument, GovDid};
pub use error::IdentityError;

/// Generate a fresh Ed25519 signing key.
///
/// The seed comes from `rand` 0.8's `OsRng`. `ed25519-dalek` 3.x requires a `rand_core` 0.10
/// RNG for `SigningKey::generate`, which `rand` 0.8's `OsRng` does not implement. Building the
/// key from 32 random bytes avoids the version mismatch.
pub(crate) fn random_signing_key() -> ed25519_dalek::SigningKey {
    use rand::RngCore;
    let mut seed = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut seed);
    ed25519_dalek::SigningKey::from_bytes(&seed)
}
