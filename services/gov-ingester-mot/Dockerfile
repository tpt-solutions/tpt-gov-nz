FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mot/ services/gov-dept-mot/
COPY services/gov-ingester-mot/ services/gov-ingester-mot/

# Build the Ministry of Transport ingester
RUN cargo build --release -p gov-ingester-mot

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-mot /usr/local/bin/gov-ingester-mot
COPY --from=builder /app/services/gov-ingester-mot/fixtures /fixtures

CMD ["gov-ingester-mot"]
