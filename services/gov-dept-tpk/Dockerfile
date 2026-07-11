FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-tpk/ services/gov-dept-tpk/

# Build only the TPK service
RUN cargo build --release -p gov-dept-tpk

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-tpk /usr/local/bin/gov-dept-tpk
COPY --from=builder /app/services/gov-dept-tpk/migrations /migrations

EXPOSE 8108 7019

CMD ["gov-dept-tpk"]
