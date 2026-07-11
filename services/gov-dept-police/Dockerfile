FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-police/ services/gov-dept-police/

# Build only the Police service
RUN cargo build --release -p gov-dept-police

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-police /usr/local/bin/gov-dept-police
COPY --from=builder /app/services/gov-dept-police/migrations /migrations

EXPOSE 8097 7008

CMD ["gov-dept-police"]
