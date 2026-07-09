FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-ird/ services/gov-dept-ird/

# Build only the IRD service
RUN cargo build --release -p gov-dept-ird

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-ird /usr/local/bin/gov-dept-ird
COPY --from=builder /app/services/gov-dept-ird/migrations /migrations

EXPOSE 8090 7001

CMD ["gov-dept-ird"]
