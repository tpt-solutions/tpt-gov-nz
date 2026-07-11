package msd.consent

# Consent policy for the Ministry of Social Development (MSD) department node.
#
# Covers MSD services beyond the Work and Income (WINZ) line already implemented
# as a separate node — here, StudyLink (student loans/allowances) and the broader
# MSD case history. Mirrors `src/consent.rs` in `gov-dept-msd`. Every
# cross-department DATA_REQUEST arriving at /citizen/data must carry signed
# DataGrantCredentials; a request is allowed only if every requested scope is
# covered by a non-expired grant whose requesting/providing departments and
# citizen DID match the request.
#
# Citizen (self) access and staff (case-worker) access are authorised directly by
# the MSD node because the citizen has implicit consent over their own record.

import rego.v1

default allow := false

allow if {
    every scope in input.requested_scopes {
        scope_covered(scope)
    }
}

scope_covered(scope) if {
    some grant in input.consent_grants
    grant.requestingDeptId == input.requesting_dept_id
    grant.providingDeptId == input.providing_dept_id
    grant.citizenDid == input.citizen_did
    grant.expiresAt > time.now_ns() / 1000000000
    scope in grant.scopes
}

denied_scopes contains scope if {
    scope := input.requested_scopes[_]
    not scope_covered(scope)
}
