FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-schema-registry/ services/gov-schema-registry/

# Build only the schema registry
RUN cargo build --release -p gov-schema-registry

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-schema-registry /usr/local/bin/gov-schema-registry
COPY --from=builder /app/services/gov-schema-registry/migrations /migrations

EXPOSE 8089

CMD ["gov-schema-registry"]
