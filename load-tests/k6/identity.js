import http from "k6/http";
import { check, sleep } from "k6";

// Load test for the identity server: DID lookup and grant listing.
//   k6 run -e BASE_URL=http://localhost:8081 load-tests/k6/identity.js

const BASE_URL = __ENV.BASE_URL || "http://localhost:8081";
const VUS = parseInt(__ENV.VUS || "20", 10);
const DURATION = __ENV.DURATION || "1m";

export const options = {
  scenarios: {
    constant: { executor: "constant-vus", vus: VUS, duration: DURATION },
  },
  thresholds: {
    http_req_failed: ["rate<0.01"],
    http_req_duration: ["p(95)<250", "p(99)<700"],
  },
};

const DIDS = [
  "did:gov:nz:test-citizen-001",
  "did:gov:nz:demo-alex-tane",
];

export default function () {
  const did = DIDS[Math.floor(Math.random() * DIDS.length)];

  const health = http.get(`${BASE_URL}/health`);
  check(health, { "identity health 200": (r) => r.status === 200 });

  const doc = http.get(`${BASE_URL}/v1/did/${encodeURIComponent(did)}`);
  check(doc, { "did document <500": (r) => r.status < 500 });

  const grants = http.get(
    `${BASE_URL}/v1/grants?citizen_did=${encodeURIComponent(did)}`,
  );
  check(grants, { "grants list <500": (r) => r.status < 500 });

  sleep(1);
}
