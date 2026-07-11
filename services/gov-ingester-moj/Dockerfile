FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-moj/ services/gov-dept-moj/
COPY services/gov-ingester-moj/ services/gov-ingester-moj/

# Build the MOJ ingester
RUN cargo build --release -p gov-ingester-moj

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-moj /usr/local/bin/gov-ingester-moj
COPY --from=builder /app/services/gov-ingester-moj/fixtures /fixtures

CMD ["gov-ingester-moj"]
