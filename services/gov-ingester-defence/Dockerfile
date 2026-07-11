FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-defence/ services/gov-dept-defence/
COPY services/gov-ingester-defence/ services/gov-ingester-defence/

# Build the Ministry of Defence ingester
RUN cargo build --release -p gov-ingester-defence

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-defence /usr/local/bin/gov-ingester-defence
COPY --from=builder /app/services/gov-ingester-defence/fixtures /fixtures

CMD ["gov-ingester-defence"]
