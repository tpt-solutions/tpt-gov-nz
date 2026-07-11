FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-gcsb/ services/gov-dept-gcsb/
COPY services/gov-ingester-gcsb/ services/gov-ingester-gcsb/

# Build the Government Communications Security Bureau ingester
RUN cargo build --release -p gov-ingester-gcsb

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-gcsb /usr/local/bin/gov-ingester-gcsb
COPY --from=builder /app/services/gov-ingester-gcsb/fixtures /fixtures

CMD ["gov-ingester-gcsb"]
