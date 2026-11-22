//! Cross-department consent verification for the IRD department node.
//!
//! This module mirrors `policies/ird.rego` so the same allow/deny decision can be
//! made locally (and in unit tests) without a running OPA sidecar. When an OPA
//! endpoint is configured via `OPA_URL`, the route layer can additionally defer to
//! the deployed policy; the local evaluation below is the canonical fallback.

use crate::error::IrdError;
use gov_consent::verify_grant;
use gov_identity_core::DataGrantCredential;

/// Input to the consent decision, structurally identical to the OPA policy input.
pub struct ConsentInput<'a> {
    pub requesting_dept_id: &'a str,
    pub citizen_did: &'a str,
    pub requested_scopes: &'a [String],
    pub consent_grants: &'a [DataGrantCredential],
}

/// Mirrors `policies/ird.rego` `allow`: every requested scope must be covered.
pub fn evaluate_allow(input: &ConsentInput) -> bool {
    input
        .requested_scopes
        .iter()
        .all(|scope| scope_covered(input, scope))
}

/// Mirrors `policies/ird.rego` `scope_covered`.
fn scope_covered(input: &ConsentInput, scope: &str) -> bool {
    let now = chrono::Utc::now().timestamp();
    input.consent_grants.iter().any(|grant| {
        grant.requesting_dept_id == input.requesting_dept_id
            && grant.providing_dept_id == "ird"
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
///
/// Verifies each grant's signature, expiry, and revocation status using the
/// shared `gov_consent::verify_grant` function. Returns [`IrdError::ScopeNotGranted`]
/// if any requested scope is not authorised or if any grant fails verification.
pub async fn verify_access<'a>(input: &ConsentInput<'a>) -> Result<(), IrdError> {
    if !evaluate_allow(input) {
        return Err(IrdError::ScopeNotGranted(denied_scopes(input).join(",")));
    }

    // Verify each grant's signature and revocation status
    for grant in input.consent_grants {
        verify_grant(grant)
            .await
            .map_err(|e| IrdError::ScopeNotGranted(format!("grant verification failed: {e}")))?;
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

    fn grant(
        issuer: &CredentialIssuer,
        requesting: &str,
        scopes: Vec<String>,
        ttl: i64,
    ) -> DataGrantCredential {
        issuer.issue_data_grant(
            gov_identity_core::GovDid::parse("did:gov:nz:test-citizen-001").unwrap(),
            requesting,
            "ird",
            scopes,
            ttl,
        )
    }

    #[test]
    fn winz_income_grant_allows_ird_income() {
        let i = issuer();
        let g = grant(&i, "winz", vec!["ird:income".into()], 3600);
        let scopes = vec!["ird:income".into()];
        let input = ConsentInput {
            requesting_dept_id: "winz",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(evaluate_allow(&input));
    }

    #[test]
    fn missing_scope_is_denied() {
        let i = issuer();
        let g = grant(&i, "winz", vec!["ird:income".into()], 3600);
        let scopes = vec!["ird:kiwisaver".into()];
        let input = ConsentInput {
            requesting_dept_id: "winz",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }

    #[test]
    fn wrong_requesting_dept_is_denied() {
        let i = issuer();
        let g = grant(&i, "moh", vec!["ird:income".into()], 3600);
        let scopes = vec!["ird:income".into()];
        let input = ConsentInput {
            requesting_dept_id: "winz",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }

    #[test]
    fn expired_grant_is_denied() {
        let i = issuer();
        let g = grant(&i, "winz", vec!["ird:income".into()], -10); // already expired
        let scopes = vec!["ird:income".into()];
        let input = ConsentInput {
            requesting_dept_id: "winz",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }
}
