FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-ird/ services/gov-dept-ird/
COPY services/gov-ingester-ird/ services/gov-ingester-ird/

# Build the IRD ingester
RUN cargo build --release -p gov-ingester-ird

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-ird /usr/local/bin/gov-ingester-ird
COPY --from=builder /app/services/gov-ingester-ird/fixtures /fixtures

CMD ["gov-ingester-ird"]
