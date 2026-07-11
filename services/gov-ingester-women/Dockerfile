FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-women/ services/gov-dept-women/
COPY services/gov-ingester-women/ services/gov-ingester-women/

# Build the Ministry for Women ingester
RUN cargo build --release -p gov-ingester-women

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-women /usr/local/bin/gov-ingester-women
COPY --from=builder /app/services/gov-ingester-women/fixtures /fixtures

CMD ["gov-ingester-women"]
