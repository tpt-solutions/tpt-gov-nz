FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-winz/ services/gov-dept-winz/

# Build only the WINZ service
RUN cargo build --release -p gov-dept-winz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-winz /usr/local/bin/gov-dept-winz
COPY --from=builder /app/services/gov-dept-winz/migrations /migrations

EXPOSE 8091 7002

CMD ["gov-dept-winz"]
