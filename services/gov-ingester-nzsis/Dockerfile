FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-nzsis/ services/gov-dept-nzsis/
COPY services/gov-ingester-nzsis/ services/gov-ingester-nzsis/

# Build the New Zealand Security Intelligence Service ingester
RUN cargo build --release -p gov-ingester-nzsis

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-nzsis /usr/local/bin/gov-ingester-nzsis
COPY --from=builder /app/services/gov-ingester-nzsis/fixtures /fixtures

CMD ["gov-ingester-nzsis"]
