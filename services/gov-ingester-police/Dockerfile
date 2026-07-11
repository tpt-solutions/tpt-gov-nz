FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-police/ services/gov-dept-police/
COPY services/gov-ingester-police/ services/gov-ingester-police/

# Build the Police ingester
RUN cargo build --release -p gov-ingester-police

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-police /usr/local/bin/gov-ingester-police
COPY --from=builder /app/services/gov-ingester-police/fixtures /fixtures

CMD ["gov-ingester-police"]
