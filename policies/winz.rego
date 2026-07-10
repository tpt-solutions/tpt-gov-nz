package winz.consent

# Consent policy for the Work and Income (MSD) department node.
#
# This policy is the source of truth deployed to the OPA sidecar that sits in
# front of the WINZ department service. The Rust `gov-dept-winz` crate mirrors the
# `allow` / `scope_covered` logic in `src/consent.rs` so the same decision can be
# made locally (and in unit tests) without a running OPA instance.
#
# Every cross-department DATA_REQUEST arriving at /citizen/data must carry one or
# more signed DataGrantCredentials (consent grants). A request is allowed only if
# *every* requested scope is covered by at least one non-expired grant whose
# requesting/providing departments and citizen DID match the request.
#
# Citizen (self) access and staff (case-worker) access are handled outside this
# policy — the WINZ node authorises those paths directly because the citizen has
# implicit consent over their own record.

import rego.v1

# Decision: is the incoming cross-department request permitted?
default allow := false

allow if {
    every scope in input.requested_scopes {
        scope_covered(scope)
    }
}

# A single scope is covered when some consent grant:
#   - was issued to the requesting department,
#   - is provided by WINZ,
#   - belongs to the same citizen,
#   - has not expired, and
#   - explicitly lists the scope.
scope_covered(scope) if {
    some grant in input.consent_grants
    grant.requestingDeptId == input.requesting_dept_id
    grant.providingDeptId == input.providing_dept_id
    grant.citizenDid == input.citizen_did
    grant.expiresAt > time.now_ns() / 1000000000
    scope in grant.scopes
}

# Helper for audit logging: the scopes that were NOT covered by any grant.
denied_scopes contains scope if {
    scope := input.requested_scopes[_]
    not scope_covered(scope)
}
