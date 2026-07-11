FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-linz/ services/gov-dept-linz/
COPY services/gov-ingester-linz/ services/gov-ingester-linz/

# Build the LINZ ingester
RUN cargo build --release -p gov-ingester-linz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-linz /usr/local/bin/gov-ingester-linz
COPY --from=builder /app/services/gov-ingester-linz/fixtures /fixtures

CMD ["gov-ingester-linz"]
