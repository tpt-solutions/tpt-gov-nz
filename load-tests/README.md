# Load tests (k6)

Run against a locally running stack (see `docker/phase1.yml` or `docker/demo.yml`).

```bash
# Install k6: https://k6.io/docs/get-started/installation/

# API gateway (routes to department services)
k6 run -e BASE_URL=http://localhost:8080 load-tests/k6/gateway.js

# Identity server (DID + grant lookups)
k6 run -e BASE_URL=http://localhost:8081 load-tests/k6/identity.js

# Department service (IRD) direct
k6 run -e BASE_URL=http://localhost:8090 load-tests/k6/dept-ird.js

# Spike / soak variants
k6 run -e BASE_URL=http://localhost:8080 -e VUS=200 -e DURATION=5m load-tests/k6/gateway.js
```

Each script sets `thresholds` for error rate (`<1%`) and latency (`p95 < 300ms`,
`p99 < 800ms` for the gateway). Tune `VUS`/`DURATION` via env vars. The
department services and gateway enforce per-department rate limiting and circuit
breakers, so watch for `429`/`503` under sustained load.
