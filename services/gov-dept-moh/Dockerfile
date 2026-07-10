FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-moh/ services/gov-dept-moh/

# Build only the MOH service
RUN cargo build --release -p gov-dept-moh

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-moh /usr/local/bin/gov-dept-moh
COPY --from=builder /app/services/gov-dept-moh/migrations /migrations

EXPOSE 8092 7003

CMD ["gov-dept-moh"]
