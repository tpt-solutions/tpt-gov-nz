FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-maritime/ services/gov-dept-maritime/
COPY services/gov-ingester-maritime/ services/gov-ingester-maritime/

# Build the Maritime New Zealand ingester
RUN cargo build --release -p gov-ingester-maritime

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-maritime /usr/local/bin/gov-ingester-maritime
COPY --from=builder /app/services/gov-ingester-maritime/fixtures /fixtures

CMD ["gov-ingester-maritime"]
