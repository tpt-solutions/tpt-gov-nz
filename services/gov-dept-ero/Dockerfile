FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-ero/ services/gov-dept-ero/

# Build only the Education Review Office service
RUN cargo build --release -p gov-dept-ero

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-ero /usr/local/bin/gov-dept-ero
COPY --from=builder /app/services/gov-dept-ero/migrations /migrations

EXPOSE 8140 7060

CMD ["gov-dept-ero"]
