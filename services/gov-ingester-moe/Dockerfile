FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-moe/ services/gov-dept-moe/
COPY services/gov-ingester-moe/ services/gov-ingester-moe/

# Build the Ministry of Education ingester
RUN cargo build --release -p gov-ingester-moe

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-moe /usr/local/bin/gov-ingester-moe
COPY --from=builder /app/services/gov-ingester-moe/fixtures /fixtures

CMD ["gov-ingester-moe"]
