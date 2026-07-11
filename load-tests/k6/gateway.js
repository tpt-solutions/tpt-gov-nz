import http from "k6/http";
import { check, sleep, Counter } from "k6";
import { Rate } from "k6/metrics";

// Load test for the API gateway. Run with:
//   k6 run -e BASE_URL=http://localhost:8080 load-tests/k6/gateway.js
// Or with staged load:
//   k6 run -e BASE_URL=http://localhost:8080 -e VUS=50 -e DURATION=2m load-tests/k6/gateway.js

const BASE_URL = __ENV.BASE_URL || "http://localhost:8080";
const VUS = parseInt(__ENV.VUS || "20", 10);
const DURATION = __ENV.DURATION || "1m";

const errorRate = new Rate("errors");
const serverErrors = new Counter("server_errors");

export const options = {
  scenarios: {
    constant: {
      executor: "constant-vus",
      vus: VUS,
      duration: DURATION,
    },
  },
  thresholds: {
    http_req_failed: ["rate<0.01"],
    http_req_duration: ["p(95)<300", "p(99)<800"],
    errors: ["rate<0.01"],
  },
};

const DIDS = [
  "did:gov:nz:test-citizen-001",
  "did:gov:nz:test-citizen-002",
  "did:gov:nz:demo-alex-tane",
];

export default function () {
  const did = DIDS[Math.floor(Math.random() * DIDS.length)];

  const health = http.get(`${BASE_URL}/health`);
  check(health, { "gateway health 200": (r) => r.status === 200 }) ||
    (errorRate.add(1), serverErrors.add(1));

  const resolve = http.get(`${BASE_URL}/v1/citizen/resolve?did=${encodeURIComponent(did)}`);
  check(resolve, { "citizen resolve ok": (r) => r.status < 500 }) ||
    (errorRate.add(1), serverErrors.add(1));

  // Route through the gateway to a department service (IRD by default).
  const dept = http.post(
    `${BASE_URL}/v1/dept/ird/citizen/data`,
    JSON.stringify({ did, scopes: ["ird:income", "ird:tax-summary"] }),
    { headers: { "Content-Type": "application/json" } },
  );
  check(dept, { "dept data ok": (r) => r.status < 500 }) ||
    (errorRate.add(1), serverErrors.add(1));

  sleep(1);
}
