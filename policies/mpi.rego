package mpi.consent

# Consent policy for the Ministry for Primary Industries (MPI) department node.
#
# Mirrors `src/consent.rs` in `gov-dept-mpi`. Every cross-department DATA_REQUEST
# arriving at /citizen/data must carry signed DataGrantCredentials; a request is
# allowed only if every requested scope is covered by a non-expired grant whose
# requesting/providing departments and citizen DID match the request.
#
# Citizen (self) access and staff (case-worker) access are authorised directly by
# the MPI node because the citizen has implicit consent over their own record.

import rego.v1

default allow := false

allow if {
    every scope in input.requested_scopes {
        scope_covered(scope)
    }
}

scope_covered(scope) if {
    some grant in input.consent_grants
    grant.requesting_dept_id == input.requesting_dept_id
    grant.providing_dept_id == input.providing_dept_id
    grant.citizen_did == input.citizen_did
    grant.expires_at > time.now_ns() / 1000000000
    scope in grant.scopes
}

denied_scopes contains scope if {
    scope := input.requested_scopes[_]
    not scope_covered(scope)
}
