FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-gcsb/ services/gov-dept-gcsb/

# Build only the Government Communications Security Bureau service
RUN cargo build --release -p gov-dept-gcsb

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-gcsb /usr/local/bin/gov-dept-gcsb
COPY --from=builder /app/services/gov-dept-gcsb/migrations /migrations

EXPOSE 8146 7066

CMD ["gov-dept-gcsb"]
