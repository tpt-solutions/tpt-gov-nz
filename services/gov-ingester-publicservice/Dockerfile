FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-publicservice/ services/gov-dept-publicservice/
COPY services/gov-ingester-publicservice/ services/gov-ingester-publicservice/

# Build the Te Kawa Mataaho Public Service Commission ingester
RUN cargo build --release -p gov-ingester-publicservice

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-publicservice /usr/local/bin/gov-ingester-publicservice
COPY --from=builder /app/services/gov-ingester-publicservice/fixtures /fixtures

CMD ["gov-ingester-publicservice"]
