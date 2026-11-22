//! Shared grant verification combining signature check + revocation check.

use crate::issuer::issuer_key_from_env;
use crate::revocation::RevocationClient;
use anyhow::{Context, Result};
use gov_identity_core::DataGrantCredential;

/// Verifies a `DataGrantCredential`:
/// 1. Checks the grant is not expired (`is_valid`)
/// 2. Verifies the Ed25519 signature against the identity server's public key
/// 3. Checks the grant has not been revoked via the identity server
///
/// Returns `Ok(())` if the grant is valid, active, and properly signed.
/// Returns an error if any check fails (fail-closed).
pub async fn verify_grant(grant: &DataGrantCredential) -> Result<()> {
    // 1. Check expiry
    grant.is_valid().context("grant expired")?;

    // 2. Verify signature against issuer key
    let issuer_key = issuer_key_from_env().context("failed to load issuer public key")?;
    grant
        .verify_signature(&issuer_key)
        .context("grant signature verification failed")?;

    // 3. Check revocation status
    let revocation_client =
        RevocationClient::from_env().context("failed to create revocation client")?;
    revocation_client
        .ensure_active(&[grant.id])
        .await
        .context("grant revoked or unknown")?;

    Ok(())
}

/// Verifies multiple grants at once (batch revocation check).
/// More efficient than calling `verify_grant` in a loop.
pub async fn verify_grants(grants: &[DataGrantCredential]) -> Result<()> {
    if grants.is_empty() {
        return Ok(());
    }

    // 1. Check expiry for all grants
    for grant in grants {
        grant.is_valid().context("grant expired")?;
    }

    // 2. Verify signatures
    let issuer_key = issuer_key_from_env().context("failed to load issuer public key")?;
    for grant in grants {
        grant
            .verify_signature(&issuer_key)
            .context("grant signature verification failed")?;
    }

    // 3. Batch revocation check
    let revocation_client =
        RevocationClient::from_env().context("failed to create revocation client")?;
    let ids: Vec<_> = grants.iter().map(|g| g.id).collect();
    revocation_client
        .ensure_active(&ids)
        .await
        .context("grant revoked or unknown")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gov_identity_core::{CredentialIssuer, GovDid};

    #[tokio::test]
    async fn test_verify_grant_fails_without_env() {
        // Without TPT__GOV__ISSUER_PUBLIC_KEY set, should fail
        let issuer = CredentialIssuer::generate();
        let citizen = GovDid::parse("did:gov:nz:citizen001").unwrap();
        let grant =
            issuer.issue_data_grant(citizen, "winz", "ird", vec!["ird:income".into()], 3600);

        // This will fail because env var is not set
        let result = verify_grant(&grant).await;
        assert!(result.is_err());
    }
}
