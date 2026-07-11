import http from "k6/http";
import { check, sleep } from "k6";

// Load test for a department service (IRD) directly.
//   k6 run -e BASE_URL=http://localhost:8090 load-tests/k6/dept-ird.js

const BASE_URL = __ENV.BASE_URL || "http://localhost:8090";
const VUS = parseInt(__ENV.VUS || "30", 10);
const DURATION = __ENV.DURATION || "1m";

export const options = {
  scenarios: {
    constant: { executor: "constant-vus", vus: VUS, duration: DURATION },
  },
  thresholds: {
    http_req_failed: ["rate<0.01"],
    http_req_duration: ["p(95)<200", "p(99)<600"],
  },
};

const DIDS = [
  "did:gov:nz:test-citizen-001",
  "did:gov:nz:demo-alex-tane",
];

export default function () {
  const did = DIDS[Math.floor(Math.random() * DIDS.length)];

  const health = http.get(`${BASE_URL}/health`);
  check(health, { "dept health 200": (r) => r.status === 200 });

  const resolve = http.post(
    `${BASE_URL}/citizen/resolve`,
    JSON.stringify({ did }),
    { headers: { "Content-Type": "application/json" } },
  );
  check(resolve, { "resolve <500": (r) => r.status < 500 });

  const data = http.post(
    `${BASE_URL}/citizen/data`,
    JSON.stringify({
      did,
      scopes: ["ird:income", "ird:tax-summary", "ird:kiwisaver", "ird:wff"],
      requesting_dept_id: "staff",
      performed_by: "staff",
    }),
    { headers: { "Content-Type": "application/json" } },
  );
  check(data, { "data <500": (r) => r.status < 500 });

  sleep(1);
}
