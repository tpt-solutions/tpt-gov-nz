//! Cross-department consent verification for the Ministry of Education department node.
//!
//! Mirrors `policies/moe.rego` so the same allow/deny decision can be made locally
//! (and in unit tests) without a running OPA sidecar.

use crate::error::MoeError;
use ed25519_dalek::VerifyingKey;
use gov_identity_core::DataGrantCredential;

pub struct ConsentInput<'a> {
    pub requesting_dept_id: &'a str,
    pub citizen_did: &'a str,
    pub requested_scopes: &'a [String],
    pub consent_grants: &'a [DataGrantCredential],
}

pub fn evaluate_allow(input: &ConsentInput) -> bool {
    input
        .requested_scopes
        .iter()
        .all(|scope| scope_covered(input, scope))
}

fn scope_covered(input: &ConsentInput, scope: &str) -> bool {
    let now = chrono::Utc::now().timestamp();
    input.consent_grants.iter().any(|grant| {
        grant.requesting_dept_id == input.requesting_dept_id
            && grant.providing_dept_id == "moe"
            && grant.citizen_did.as_str() == input.citizen_did
            && grant.expires_at > now
            && grant.scopes.iter().any(|s| s == scope)
    })
}

pub fn denied_scopes(input: &ConsentInput) -> Vec<String> {
    input
        .requested_scopes
        .iter()
        .filter(|scope| !scope_covered(input, scope))
        .cloned()
        .collect()
}

pub fn verify_access(
    input: &ConsentInput,
    issuer_key: Option<&VerifyingKey>,
) -> Result<(), MoeError> {
    if !evaluate_allow(input) {
        return Err(MoeError::ScopeNotGranted(denied_scopes(input).join(",")));
    }

    if let Some(key) = issuer_key {
        let all_signed = input.requested_scopes.iter().all(|scope| {
            input.consent_grants.iter().any(|grant| {
                grant.scopes.iter().any(|s| s == scope)
                    && grant.requesting_dept_id == input.requesting_dept_id
                    && grant.providing_dept_id == "moe"
                    && grant.citizen_did.as_str() == input.citizen_did
                    && grant.expires_at > chrono::Utc::now().timestamp()
                    && grant.verify_signature(key).is_ok()
            })
        });
        if !all_signed {
            return Err(MoeError::ScopeNotGranted("grant signature invalid".into()));
        }
    }

    Ok(())
}


/// Enforce cross-department grant checks: verify each grant's Ed25519 signature
/// against the identity server's issuer key (scope/expiry/party checks included),
/// then confirm none of the presented grants has been revoked. Fails closed.
pub async fn enforce_signed_and_unrevoked(input: &ConsentInput<'_>) -> Result<(), MoeError> {
    let key = gov_consent::issuer_key_from_env()
        .map_err(|e| MoeError::ScopeNotGranted(format!("issuer key unavailable: {e}")))?;
    verify_access(input, Some(&key))?;
    let ids: Vec<_> = input.consent_grants.iter().map(|g| g.id).collect();
    let revocation = gov_consent::RevocationClient::from_env()
        .map_err(|e| MoeError::ScopeNotGranted(format!("revocation check unavailable: {e}")))?;
    revocation
        .ensure_active(&ids)
        .await
        .map_err(|e| MoeError::ScopeNotGranted(format!("consent grant revoked or unknown: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gov_identity_core::CredentialIssuer;

    fn issuer() -> CredentialIssuer {
        CredentialIssuer::generate()
    }

    fn grant(
        issuer: &CredentialIssuer,
        requesting: &str,
        scopes: Vec<String>,
    ) -> DataGrantCredential {
        issuer.issue_data_grant(
            gov_identity_core::GovDid::parse("did:gov:nz:test-citizen-001").unwrap(),
            requesting,
            "moe",
            scopes,
            3600,
        )
    }

    #[test]
    fn moe_grant_allows_moe_scope() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["moe:enrolment".into()]);
        let scopes = vec!["moe:enrolment".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(evaluate_allow(&input));
        assert!(verify_access(&input, Some(&i.verifying_key())).is_ok());
    }

    #[test]
    fn wrong_dept_grant_denies() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["ird:income".into()]);
        let scopes = vec!["moe:enrolment".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }

    #[test]
    fn expired_grant_is_denied() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["moe:enrolment".into()]);
        let mut g2 = g;
        g2.expires_at = chrono::Utc::now().timestamp() - 10;
        let scopes = vec!["moe:enrolment".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g2],
        };
        assert!(!evaluate_allow(&input));
    }
}
