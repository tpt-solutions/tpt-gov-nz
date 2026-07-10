FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-winz/ services/gov-dept-winz/
COPY services/gov-ingester-winz/ services/gov-ingester-winz/

# Build the WINZ ingester
RUN cargo build --release -p gov-ingester-winz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-winz /usr/local/bin/gov-ingester-winz
COPY --from=builder /app/services/gov-ingester-winz/fixtures /fixtures

CMD ["gov-ingester-winz"]
