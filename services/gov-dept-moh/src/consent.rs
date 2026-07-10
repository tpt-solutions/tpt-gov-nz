//! Cross-department consent verification for the MOH department node.
//!
//! This module mirrors `policies/moh.rego` so the same allow/deny decision can be
//! made locally (and in unit tests) without a running OPA sidecar. When an OPA
//! endpoint is configured via `OPA_URL`, the route layer can additionally defer to
//! the deployed policy; the local evaluation below is the canonical fallback.

use crate::error::MohError;
use ed25519_dalek::VerifyingKey;
use gov_identity_core::DataGrantCredential;

/// Input to the consent decision, structurally identical to the OPA policy input.
pub struct ConsentInput<'a> {
    pub requesting_dept_id: &'a str,
    pub citizen_did: &'a str,
    pub requested_scopes: &'a [String],
    pub consent_grants: &'a [DataGrantCredential],
}

/// Mirrors `policies/moh.rego` `allow`: every requested scope must be covered.
pub fn evaluate_allow(input: &ConsentInput) -> bool {
    input.requested_scopes.iter().all(|scope| scope_covered(input, scope))
}

/// Mirrors `policies/moh.rego` `scope_covered`.
fn scope_covered(input: &ConsentInput, scope: &str) -> bool {
    let now = chrono::Utc::now().timestamp();
    input.consent_grants.iter().any(|grant| {
        grant.requesting_dept_id == input.requesting_dept_id
            && grant.providing_dept_id == "moh"
            && grant.citizen_did.as_str() == input.citizen_did
            && grant.expires_at > now
            && grant.scopes.iter().any(|s| s == scope)
    })
}

/// The scopes in `input` that are NOT covered by any grant (audit helper).
pub fn denied_scopes(input: &ConsentInput) -> Vec<String> {
    input
        .requested_scopes
        .iter()
        .filter(|scope| !scope_covered(input, scope))
        .cloned()
        .collect()
}

/// Authorise a cross-department request.
pub fn verify_access(
    input: &ConsentInput,
    issuer_key: Option<&VerifyingKey>,
) -> Result<(), MohError> {
    if !evaluate_allow(input) {
        return Err(MohError::ScopeNotGranted(denied_scopes(input).join(",")));
    }

    if let Some(key) = issuer_key {
        let all_signed = input.requested_scopes.iter().all(|scope| {
            input.consent_grants.iter().any(|grant| {
                grant.scopes.iter().any(|s| s == scope)
                    && grant.requesting_dept_id == input.requesting_dept_id
                    && grant.providing_dept_id == "moh"
                    && grant.citizen_did.as_str() == input.citizen_did
                    && grant.expires_at > chrono::Utc::now().timestamp()
                    && grant.verify_signature(key).is_ok()
            })
        });
        if !all_signed {
            return Err(MohError::ScopeNotGranted("grant signature invalid".into()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gov_identity_core::CredentialIssuer;

    fn issuer() -> CredentialIssuer {
        CredentialIssuer::generate()
    }

    fn grant(issuer: &CredentialIssuer, requesting: &str, scopes: Vec<String>, ttl: i64) -> DataGrantCredential {
        issuer.issue_data_grant(
            gov_identity_core::GovDid::parse("did:gov:nz:test-citizen-001").unwrap(),
            requesting,
            "moh",
            scopes,
            3600,
        )
    }

    #[test]
    fn ird_grant_denies_moh_scopes() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["ird:income".into()], 3600);
        let scopes = vec!["moh:vaccinations".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }

    #[test]
    fn moh_grant_allows_moh_scope() {
        let i = issuer();
        let g = grant(&i, "moh", vec!["moh:vaccinations".into()], 3600);
        let scopes = vec!["moh:vaccinations".into()];
        let input = ConsentInput {
            requesting_dept_id: "moh",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(evaluate_allow(&input));
        assert!(verify_access(&input, Some(&i.verifying_key())).is_ok());
    }

    #[test]
    fn missing_scope_is_denied() {
        let i = issuer();
        let g = grant(&i, "moh", vec!["moh:gp".into()], 3600);
        let scopes = vec!["moh:vaccinations".into()];
        let input = ConsentInput {
            requesting_dept_id: "moh",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }

    #[test]
    fn expired_grant_is_denied() {
        let i = issuer();
        let g = grant(&i, "moh", vec!["moh:vaccinations".into()], -10);
        let scopes = vec!["moh:vaccinations".into()];
        let input = ConsentInput {
            requesting_dept_id: "moh",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }
}
