FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-moh/ services/gov-dept-moh/
COPY services/gov-ingester-moh/ services/gov-ingester-moh/

# Build the MOH ingester
RUN cargo build --release -p gov-ingester-moh

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-moh /usr/local/bin/gov-ingester-moh
COPY --from=builder /app/services/gov-ingester-moh/fixtures /fixtures

CMD ["gov-ingester-moh"]
