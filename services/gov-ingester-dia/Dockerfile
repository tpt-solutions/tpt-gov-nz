FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-dia/ services/gov-dept-dia/
COPY services/gov-ingester-dia/ services/gov-ingester-dia/

# Build the DIA ingester
RUN cargo build --release -p gov-ingester-dia

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-dia /usr/local/bin/gov-ingester-dia
COPY --from=builder /app/services/gov-ingester-dia/fixtures /fixtures

CMD ["gov-ingester-dia"]
