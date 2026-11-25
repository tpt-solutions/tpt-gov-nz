//! Attenuated capability chains over data grants.
//!
//! This is the verification half of the capability model in
//! `docs/adr/0001-capability-token-format.md`. The root is a `DataGrantCredential`, issued
//! and signed by the identity server and bound to a holder key. The holder can delegate a
//! narrower capability by signing a link with that key, and each link binds the next
//! holder's key, so a chain can keep going.
//!
//! `verify_chain` enforces:
//! - the root is issuer-signed and unexpired;
//! - a root with no holder key cannot delegate;
//! - each link is signed by the key bound by the element before it;
//! - each link narrows its parent: scopes are a subset, expiry is not later, and the link
//!   has not expired;
//! - each link commits to the signature before it, so links cannot be reordered or spliced.
//!
//! `verify_chain_with_revocation` also rejects any chain whose root or link id the caller
//! reports as revoked. The caller supplies the revocation check, so this crate stays free of
//! storage.
//!
//! Not covered yet: purpose and max-use caveats, and dept nodes accepting only chains. See
//! `TODO.md`.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{credential::DataGrantCredential, did::GovDid, error::IdentityError};

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum CapabilityError {
    /// The root grant failed the issuer signature or expiry check. Holds the message, so the
    /// error stays comparable.
    #[error("root grant rejected: {0}")]
    Root(String),
    #[error("root grant is not bound to a holder key, so it cannot delegate")]
    Unbound,
    #[error("link {0}: holder key is missing or malformed")]
    BadHolderKey(usize),
    #[error("link {0}: not signed by the key that delegated it")]
    BadSignature(usize),
    #[error("link {0}: expired")]
    Expired(usize),
    #[error("link {0}: has no scopes")]
    NoScopes(usize),
    #[error("link {0}: scope {1} is not held by its parent")]
    Widened(usize, String),
    #[error("link {0}: expires after its parent")]
    Outlives(usize),
    #[error("root grant has been revoked")]
    RootRevoked,
    #[error("link {0}: has been revoked")]
    Revoked(usize),
}

/// One delegation step: the holder of the previous element authorises a narrower capability
/// for `holder_public_key`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLink {
    pub id: Uuid,
    pub scopes: Vec<String>,
    pub expires_at: i64,
    /// Base64 Ed25519 key of the holder this link delegates to. The chain continues with it.
    pub holder_public_key: String,
    /// Base64 Ed25519 signature by the previous holder over the canonical link payload.
    pub signature: String,
}

#[derive(Serialize)]
struct LinkSignPayload<'a> {
    id: &'a Uuid,
    previous_signature: &'a str,
    scopes: &'a [String],
    expires_at: i64,
    holder_public_key: &'a str,
}

fn link_hash(
    id: &Uuid,
    previous_signature: &str,
    scopes: &[String],
    expires_at: i64,
    holder_public_key: &str,
) -> [u8; 32] {
    let payload = LinkSignPayload {
        id,
        previous_signature,
        scopes,
        expires_at,
        holder_public_key,
    };
    let json = serde_json::to_vec(&payload).expect("serialisation of link payload must not fail");
    Sha256::digest(&json).into()
}

/// A root grant followed by zero or more delegation links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityChain {
    pub root: DataGrantCredential,
    pub links: Vec<CapabilityLink>,
}

/// What a verified chain authorises: the narrowest scopes and the earliest expiry along it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveCapability {
    pub citizen_did: GovDid,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub scopes: Vec<String>,
    pub expires_at: i64,
    /// Number of delegation links after the root.
    pub depth: usize,
}

/// A holder's Ed25519 key, used to delegate capabilities.
pub struct HolderKey(SigningKey);

impl HolderKey {
    pub fn generate() -> Self {
        Self(crate::random_signing_key())
    }

    /// Base64 public key. Put this in a grant's or link's `holder_public_key`.
    pub fn public_key_b64(&self) -> String {
        STANDARD.encode(self.0.verifying_key().as_bytes())
    }

    /// Delegate a narrower capability to `to_holder_public_key`.
    ///
    /// `parent_signature` is the signature of the element being extended: the root's
    /// `signature` for the first link, or the previous link's `signature` after that. Signing
    /// it into the link is what stops links being reordered or spliced.
    pub fn delegate(
        &self,
        parent_signature: &str,
        scopes: Vec<String>,
        expires_at: i64,
        to_holder_public_key: String,
    ) -> CapabilityLink {
        let id = Uuid::new_v4();
        let hash = link_hash(
            &id,
            parent_signature,
            &scopes,
            expires_at,
            &to_holder_public_key,
        );
        let signature = STANDARD.encode(self.0.sign(&hash).to_bytes());
        CapabilityLink {
            id,
            scopes,
            expires_at,
            holder_public_key: to_holder_public_key,
            signature,
        }
    }
}

fn parse_verifying_key(b64: &str) -> Option<VerifyingKey> {
    let bytes = STANDARD.decode(b64).ok()?;
    let arr: [u8; 32] = bytes.try_into().ok()?;
    VerifyingKey::from_bytes(&arr).ok()
}

fn parse_signature(b64: &str) -> Option<Signature> {
    let bytes = STANDARD.decode(b64).ok()?;
    let arr: [u8; 64] = bytes.try_into().ok()?;
    Some(Signature::from_bytes(&arr))
}

/// Verify a capability chain against the identity server's public key at time `now` (Unix
/// seconds). On success, returns what the chain authorises.
///
/// This does not check revocation. Use `verify_chain_with_revocation` where revocation applies.
pub fn verify_chain(
    chain: &CapabilityChain,
    issuer_key: &VerifyingKey,
    now: i64,
) -> Result<EffectiveCapability, CapabilityError> {
    verify_chain_with_revocation(chain, issuer_key, now, &|_| false)
}

/// As `verify_chain`, and also rejects the chain if the root or any link id is reported as
/// revoked by `is_revoked`. A revoked id anywhere in the chain revokes everything after it.
pub fn verify_chain_with_revocation(
    chain: &CapabilityChain,
    issuer_key: &VerifyingKey,
    now: i64,
    is_revoked: &dyn Fn(&Uuid) -> bool,
) -> Result<EffectiveCapability, CapabilityError> {
    let root = &chain.root;
    root.verify_signature(issuer_key)
        .map_err(|e| CapabilityError::Root(e.to_string()))?;
    if now > root.expires_at {
        return Err(CapabilityError::Root(IdentityError::Expired.to_string()));
    }
    if is_revoked(&root.id) {
        return Err(CapabilityError::RootRevoked);
    }

    let mut scopes = root.scopes.clone();
    let mut expires_at = root.expires_at;
    let mut previous_signature = root.signature.clone();
    // The key allowed to sign the next link: the root's holder, then each link's holder.
    let mut signer: Option<String> = root.holder_public_key.clone();

    for (index, link) in chain.links.iter().enumerate() {
        let signer_b64 = signer.as_deref().ok_or(CapabilityError::Unbound)?;
        let signer_key =
            parse_verifying_key(signer_b64).ok_or(CapabilityError::BadHolderKey(index))?;

        if is_revoked(&link.id) {
            return Err(CapabilityError::Revoked(index));
        }
        if now > link.expires_at {
            return Err(CapabilityError::Expired(index));
        }
        if link.scopes.is_empty() {
            return Err(CapabilityError::NoScopes(index));
        }
        if link.expires_at > expires_at {
            return Err(CapabilityError::Outlives(index));
        }
        if let Some(widened) = link.scopes.iter().find(|s| !scopes.contains(s)) {
            return Err(CapabilityError::Widened(index, widened.clone()));
        }

        let hash = link_hash(
            &link.id,
            &previous_signature,
            &link.scopes,
            link.expires_at,
            &link.holder_public_key,
        );
        let sig = parse_signature(&link.signature).ok_or(CapabilityError::BadSignature(index))?;
        signer_key
            .verify(&hash, &sig)
            .map_err(|_| CapabilityError::BadSignature(index))?;

        // The next signer must be a well-formed key, so a malformed holder is refused here.
        parse_verifying_key(&link.holder_public_key).ok_or(CapabilityError::BadHolderKey(index))?;

        scopes = link.scopes.clone();
        expires_at = link.expires_at;
        previous_signature = link.signature.clone();
        signer = Some(link.holder_public_key.clone());
    }

    Ok(EffectiveCapability {
        citizen_did: root.citizen_did.clone(),
        requesting_dept_id: root.requesting_dept_id.clone(),
        providing_dept_id: root.providing_dept_id.clone(),
        scopes,
        expires_at,
        depth: chain.links.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credential::CredentialIssuer;

    const CITIZEN: &str = "did:gov:nz:citizen-cap-001";

    struct Fixture {
        issuer: CredentialIssuer,
        a: HolderKey,
        b: HolderKey,
        c: HolderKey,
        root: DataGrantCredential,
        now: i64,
    }

    fn fixture() -> Fixture {
        let issuer = CredentialIssuer::generate();
        let a = HolderKey::generate();
        let b = HolderKey::generate();
        let c = HolderKey::generate();
        let root = issuer.issue_holder_bound_data_grant(
            GovDid::parse(CITIZEN).unwrap(),
            "winz",
            "ird",
            vec!["ird:income".into(), "ird:tax-summary".into()],
            3600,
            a.public_key_b64(),
        );
        Fixture {
            issuer,
            a,
            b,
            c,
            root,
            now: chrono::Utc::now().timestamp(),
        }
    }

    fn chain(root: &DataGrantCredential, links: Vec<CapabilityLink>) -> CapabilityChain {
        CapabilityChain {
            root: root.clone(),
            links,
        }
    }

    #[test]
    fn root_with_no_links_authorises_the_root_scopes() {
        let f = fixture();
        let eff = verify_chain(&chain(&f.root, vec![]), &f.issuer.verifying_key(), f.now).unwrap();
        assert_eq!(eff.scopes, f.root.scopes);
        assert_eq!(eff.depth, 0);
        assert_eq!(eff.requesting_dept_id, "winz");
    }

    #[test]
    fn delegation_can_narrow_scopes_and_expiry() {
        let f = fixture();
        let link = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 60,
            f.b.public_key_b64(),
        );
        let eff = verify_chain(
            &chain(&f.root, vec![link.clone()]),
            &f.issuer.verifying_key(),
            f.now,
        )
        .unwrap();
        assert_eq!(eff.scopes, vec!["ird:income".to_string()]);
        assert_eq!(eff.expires_at, link.expires_at);
        assert_eq!(eff.depth, 1);
    }

    #[test]
    fn a_chain_of_two_delegations_verifies() {
        let f = fixture();
        let l1 = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into(), "ird:tax-summary".into()],
            f.root.expires_at - 60,
            f.b.public_key_b64(),
        );
        let l2 = f.b.delegate(
            &l1.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 120,
            f.c.public_key_b64(),
        );
        let eff = verify_chain(
            &chain(&f.root, vec![l1, l2]),
            &f.issuer.verifying_key(),
            f.now,
        )
        .unwrap();
        assert_eq!(eff.scopes, vec!["ird:income".to_string()]);
        assert_eq!(eff.depth, 2);
    }

    #[test]
    fn widening_scopes_is_rejected() {
        let f = fixture();
        let link = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into(), "ird:kiwisaver".into()],
            f.root.expires_at - 60,
            f.b.public_key_b64(),
        );
        assert_eq!(
            verify_chain(
                &chain(&f.root, vec![link]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::Widened(0, "ird:kiwisaver".into()))
        );
    }

    #[test]
    fn widening_in_a_later_link_is_rejected_against_its_parent() {
        let f = fixture();
        let l1 = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 60,
            f.b.public_key_b64(),
        );
        // l1 holds only ird:income, so l2 cannot bring back ird:tax-summary.
        let l2 = f.b.delegate(
            &l1.signature,
            vec!["ird:tax-summary".into()],
            f.root.expires_at - 120,
            f.c.public_key_b64(),
        );
        assert_eq!(
            verify_chain(
                &chain(&f.root, vec![l1, l2]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::Widened(1, "ird:tax-summary".into()))
        );
    }

    #[test]
    fn a_link_cannot_outlive_its_parent() {
        let f = fixture();
        let link = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at + 10,
            f.b.public_key_b64(),
        );
        assert_eq!(
            verify_chain(
                &chain(&f.root, vec![link]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::Outlives(0))
        );
    }

    #[test]
    fn an_expired_link_is_rejected() {
        let f = fixture();
        let link = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.now - 1,
            f.b.public_key_b64(),
        );
        assert_eq!(
            verify_chain(
                &chain(&f.root, vec![link]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::Expired(0))
        );
    }

    #[test]
    fn a_link_signed_by_someone_other_than_the_bound_holder_is_rejected() {
        let f = fixture();
        // b is not the root's holder (a is), so b's signature must not verify.
        let forged = f.b.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 60,
            f.c.public_key_b64(),
        );
        assert_eq!(
            verify_chain(
                &chain(&f.root, vec![forged]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::BadSignature(0))
        );
    }

    #[test]
    fn a_link_that_commits_to_the_wrong_parent_is_rejected() {
        let f = fixture();
        let l1 = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 60,
            f.b.public_key_b64(),
        );
        // l2 was signed as if it extended the root directly, so it cannot follow l1.
        let l2 = f.b.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 120,
            f.c.public_key_b64(),
        );
        assert_eq!(
            verify_chain(
                &chain(&f.root, vec![l1, l2]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::BadSignature(1))
        );
    }

    #[test]
    fn reordered_links_are_rejected() {
        let f = fixture();
        let l1 = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 60,
            f.b.public_key_b64(),
        );
        let l2 = f.b.delegate(
            &l1.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 120,
            f.c.public_key_b64(),
        );
        assert!(matches!(
            verify_chain(
                &chain(&f.root, vec![l2, l1]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::BadSignature(0))
        ));
    }

    #[test]
    fn a_root_without_a_holder_cannot_delegate() {
        let f = fixture();
        let unbound = f.issuer.issue_data_grant(
            GovDid::parse(CITIZEN).unwrap(),
            "winz",
            "ird",
            vec!["ird:income".into()],
            3600,
        );
        let link = f.a.delegate(
            &unbound.signature,
            vec!["ird:income".into()],
            unbound.expires_at - 60,
            f.b.public_key_b64(),
        );
        assert_eq!(
            verify_chain(
                &chain(&unbound, vec![link]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::Unbound)
        );
    }

    #[test]
    fn a_root_from_another_issuer_is_rejected() {
        let f = fixture();
        let other = CredentialIssuer::generate();
        assert!(matches!(
            verify_chain(&chain(&f.root, vec![]), &other.verifying_key(), f.now),
            Err(CapabilityError::Root(_))
        ));
    }

    #[test]
    fn swapping_the_holder_key_on_a_root_breaks_the_issuer_signature() {
        let f = fixture();
        let mut tampered = f.root.clone();
        tampered.holder_public_key = Some(f.c.public_key_b64());
        assert!(matches!(
            verify_chain(&chain(&tampered, vec![]), &f.issuer.verifying_key(), f.now),
            Err(CapabilityError::Root(_))
        ));
    }

    #[test]
    fn widening_the_root_scopes_breaks_the_issuer_signature() {
        let f = fixture();
        let mut tampered = f.root.clone();
        tampered.scopes.push("ird:kiwisaver".into());
        assert!(matches!(
            verify_chain(&chain(&tampered, vec![]), &f.issuer.verifying_key(), f.now),
            Err(CapabilityError::Root(_))
        ));
    }

    #[test]
    fn a_malformed_holder_key_is_refused() {
        let f = fixture();
        let link = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 60,
            "not-a-key".into(),
        );
        assert_eq!(
            verify_chain(
                &chain(&f.root, vec![link]),
                &f.issuer.verifying_key(),
                f.now
            ),
            Err(CapabilityError::BadHolderKey(0))
        );
    }

    #[test]
    fn a_revoked_root_is_rejected() {
        let f = fixture();
        let revoked = f.root.id;
        assert_eq!(
            verify_chain_with_revocation(
                &chain(&f.root, vec![]),
                &f.issuer.verifying_key(),
                f.now,
                &|id| *id == revoked,
            ),
            Err(CapabilityError::RootRevoked)
        );
    }

    #[test]
    fn a_revoked_link_rejects_the_link_and_everything_after_it() {
        let f = fixture();
        let first = f.a.delegate(
            &f.root.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 60,
            f.b.public_key_b64(),
        );
        let second = f.b.delegate(
            &first.signature,
            vec!["ird:income".into()],
            f.root.expires_at - 120,
            f.c.public_key_b64(),
        );
        let revoked = first.id;
        let links = vec![first.clone(), second];
        let c = chain(&f.root, links);
        assert_eq!(
            verify_chain_with_revocation(&c, &f.issuer.verifying_key(), f.now, &|id| *id
                == revoked),
            Err(CapabilityError::Revoked(0))
        );
        // Nothing revoked: the same chain is still valid.
        assert!(
            verify_chain_with_revocation(&c, &f.issuer.verifying_key(), f.now, &|_| false)
                .is_ok()
        );
    }

    #[test]
    fn v1_grants_serialise_without_the_holder_field() {
        // Grants issued before holder binding must not change shape on the wire.
        let issuer = CredentialIssuer::generate();
        let v1 = issuer.issue_data_grant(
            GovDid::parse(CITIZEN).unwrap(),
            "winz",
            "ird",
            vec!["ird:income".into()],
            3600,
        );
        let json = serde_json::to_value(&v1).unwrap();
        assert!(json.get("holder_public_key").is_none());
        assert!(v1.verify_signature(&issuer.verifying_key()).is_ok());
    }
}
