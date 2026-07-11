FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-sfo/ services/gov-dept-sfo/

# Build only the Serious Fraud Office service
RUN cargo build --release -p gov-dept-sfo

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-sfo /usr/local/bin/gov-dept-sfo
COPY --from=builder /app/services/gov-dept-sfo/migrations /migrations

EXPOSE 8124 7044

CMD ["gov-dept-sfo"]
