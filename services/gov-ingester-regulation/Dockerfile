FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-regulation/ services/gov-dept-regulation/
COPY services/gov-ingester-regulation/ services/gov-ingester-regulation/

# Build the Ministry for Regulation ingester
RUN cargo build --release -p gov-ingester-regulation

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-regulation /usr/local/bin/gov-ingester-regulation
COPY --from=builder /app/services/gov-ingester-regulation/fixtures /fixtures

CMD ["gov-ingester-regulation"]
