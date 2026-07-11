FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-tec/ services/gov-dept-tec/
COPY services/gov-ingester-tec/ services/gov-ingester-tec/

# Build the Tertiary Education Commission ingester
RUN cargo build --release -p gov-ingester-tec

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-tec /usr/local/bin/gov-ingester-tec
COPY --from=builder /app/services/gov-ingester-tec/fixtures /fixtures

CMD ["gov-ingester-tec"]
