FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-publicservice/ services/gov-dept-publicservice/

# Build only the Te Kawa Mataaho Public Service Commission service
RUN cargo build --release -p gov-dept-publicservice

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-publicservice /usr/local/bin/gov-dept-publicservice
COPY --from=builder /app/services/gov-dept-publicservice/migrations /migrations

EXPOSE 8122 7042

CMD ["gov-dept-publicservice"]
