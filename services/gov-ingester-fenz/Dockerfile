FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-fenz/ services/gov-dept-fenz/
COPY services/gov-ingester-fenz/ services/gov-ingester-fenz/

# Build the Fire and Emergency New Zealand ingester
RUN cargo build --release -p gov-ingester-fenz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-fenz /usr/local/bin/gov-ingester-fenz
COPY --from=builder /app/services/gov-ingester-fenz/fixtures /fixtures

CMD ["gov-ingester-fenz"]
