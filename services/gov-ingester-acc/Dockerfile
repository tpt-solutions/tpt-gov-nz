FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-acc/ services/gov-dept-acc/
COPY services/gov-ingester-acc/ services/gov-ingester-acc/

# Build the ACC ingester
RUN cargo build --release -p gov-ingester-acc

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-acc /usr/local/bin/gov-ingester-acc
COPY --from=builder /app/services/gov-ingester-acc/fixtures /fixtures

CMD ["gov-ingester-acc"]
