FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-identity-server/ services/gov-identity-server/

# Build only the identity server
RUN cargo build --release -p gov-identity-server

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-identity-server /usr/local/bin/gov-identity-server
COPY --from=builder /app/services/gov-identity-server/migrations /migrations

EXPOSE 8081

CMD ["gov-identity-server"]
