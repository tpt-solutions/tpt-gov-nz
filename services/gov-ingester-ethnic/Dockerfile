FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-ethnic/ services/gov-dept-ethnic/
COPY services/gov-ingester-ethnic/ services/gov-ingester-ethnic/

# Build the Ministry for Ethnic Communities ingester
RUN cargo build --release -p gov-ingester-ethnic

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-ethnic /usr/local/bin/gov-ingester-ethnic
COPY --from=builder /app/services/gov-ingester-ethnic/fixtures /fixtures

CMD ["gov-ingester-ethnic"]
