# 0002. Department node access control: mTLS from gateway and staff portal

- **Status:** Accepted. Partly implemented: step 1 and the IRD service (step 2, for IRD only).
- **Date:** 2026-11-23
- **Decides:** the "Remove `requesting_dept_id` self-assertion bypass" item in `TODO.md`

## Context

Department nodes currently trust the caller's claim about who is asking:

- A request with no `requesting_dept_id` (in the header set by the gateway or in the
  body) is treated as **direct access**. Direct access skips consent checks.
- The staff portal calls department nodes directly, with no credential, so direct access
  is what makes staff access work today.
- The gateway already sets `x-requesting-dept-id` from the caller's JWT and drops
  caller-supplied identity headers, but a request that reaches a department node
  without going through the gateway can set or omit any of these fields.

Network reachability is therefore the real control. Anyone who can reach a department
node can read its data, and no header or body field can be trusted to stop that.

## Decision

1. **Department nodes accept requests only from authenticated peers.** A node requires
   a client certificate issued by the platform internal CA (`crates/gov-mtls`). Requests
   without one are refused at the TLS layer.
2. **Two peers are allowed:** the gateway (for citizen and cross-department traffic) and
   the staff portal (for case-worker traffic). Each holds its own client certificate.
   Staff identity still comes from the staff session token, which the portal forwards
   and the node verifies.
3. **Identity headers are informational once mTLS is enforced.** Nodes use the
   certificate's identity to decide whether a request is gateway or staff traffic. The
   `requesting_dept_id` value is used only for cross-department consent checks, and only
   the gateway can set it.
4. **Local development** may run without mTLS, but only with `TPT__GOV__ENV=development`
   set. A node started without mTLS and without that flag refuses to start.

## Consequences

- Each department service needs a TLS server (not just the client already in
  `gov-mtls`). The shared TLS serving code belongs in `gov-mtls`, so all services use one
  implementation.
- The staff portal needs a client certificate and a server-side HTTPS client for
  department calls.
- Compose files and `.env.example` gain certificate paths and the development flag.
- The generator and the 47 copied department services need the same change. Do this
  after the generator drift is reconciled (see `TODO.md`), so it is done once.
- The `requesting_dept_id` checks in each service stay as defence in depth.

## Alternatives considered

| Option | Why not chosen |
|---|---|
| Signed gateway token on each request | Still needs network isolation to be meaningful, and adds a token-signing key to every node. |
| Keep direct access, and rely on network policy | Puts the whole control in infrastructure that each agency configures differently. |
| Route staff through the gateway | Good long-term target, but it is a larger change to the staff portal than this decision needs. Can be done later, on top of mTLS. |

## Follow-up

**Done:** (1) `gov-mtls::serve` serves an axum router over mTLS, exposes the peer certificate fingerprint to handlers, and enforces a caller allow-list (`restrict_to_peers`). `transport_from_env` refuses to start without mTLS unless `TPT__GOV__ENV=development`. The IRD service uses it; its dev compose entry sets the development flag. Tested with real TLS handshakes (`crates/gov-mtls/tests/serve_mtls.rs`).

**Decision (amendment, 2026-11-24):** citizen-portal calls to department services go through the gateway, not directly. This replaces the earlier open question. The citizen portal is not a peer. Its requests carry a gateway token, which the gateway verifies, and the gateway is the only peer that calls the department.

- The citizen portal mints a short-lived gateway token (HS256, `TPT__GOV__JWT_SECRET`) with `sub` = the citizen DID, `scopes` = the granted scopes, and `typ: "citizen"`. The portal's session token keeps its own secret and is not sent to the gateway.
- The gateway sets `x-requesting-dept-id: citizen` for `typ: "citizen"` tokens, and `x-citizen-did` from `sub`. The scope check on the requested department still applies.
- A department node treats `citizen` as self-access only when `x-citizen-did` equals the record being read. Otherwise it refuses with 403 and still writes the audit row. The IRD service does this now.

Implemented so far: the gateway `typ` claim and requesting-department rule (with tests), and the IRD subject check (with tests). **Not yet done:** the portal token minting, routing the 94 portal call sites through `/v1/dept/{id}`, and the same subject check in the template and other services.

**Not yet done:** the staff portal and gateway client side, compose wiring beyond IRD, the template and the other 46 services, and removing the direct-access branch.


Implementation order: (1) TLS serving in `gov-mtls`, (2) dept node template and
generator, (3) gateway and staff portal clients, (4) compose and env wiring, (5) remove
the direct-access branch once the staff portal uses mTLS.
