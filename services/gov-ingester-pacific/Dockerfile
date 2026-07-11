FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-pacific/ services/gov-dept-pacific/
COPY services/gov-ingester-pacific/ services/gov-ingester-pacific/

# Build the Ministry for Pacific Peoples ingester
RUN cargo build --release -p gov-ingester-pacific

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-pacific /usr/local/bin/gov-ingester-pacific
COPY --from=builder /app/services/gov-ingester-pacific/fixtures /fixtures

CMD ["gov-ingester-pacific"]
