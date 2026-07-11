FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-acc/ services/gov-dept-acc/

# Build only the ACC service
RUN cargo build --release -p gov-dept-acc

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-acc /usr/local/bin/gov-dept-acc
COPY --from=builder /app/services/gov-dept-acc/migrations /migrations

EXPOSE 8095 7006

CMD ["gov-dept-acc"]
