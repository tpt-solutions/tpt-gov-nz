FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-regulation/ services/gov-dept-regulation/

# Build only the Ministry for Regulation service
RUN cargo build --release -p gov-dept-regulation

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-regulation /usr/local/bin/gov-dept-regulation
COPY --from=builder /app/services/gov-dept-regulation/migrations /migrations

EXPOSE 8130 7050

CMD ["gov-dept-regulation"]
