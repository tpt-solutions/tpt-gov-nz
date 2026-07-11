FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-ero/ services/gov-dept-ero/
COPY services/gov-ingester-ero/ services/gov-ingester-ero/

# Build the Education Review Office ingester
RUN cargo build --release -p gov-ingester-ero

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-ero /usr/local/bin/gov-ingester-ero
COPY --from=builder /app/services/gov-ingester-ero/fixtures /fixtures

CMD ["gov-ingester-ero"]
