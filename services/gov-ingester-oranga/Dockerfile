FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-oranga/ services/gov-dept-oranga/
COPY services/gov-ingester-oranga/ services/gov-ingester-oranga/

# Build the Oranga Tamariki ingester
RUN cargo build --release -p gov-ingester-oranga

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-oranga /usr/local/bin/gov-ingester-oranga
COPY --from=builder /app/services/gov-ingester-oranga/fixtures /fixtures

CMD ["gov-ingester-oranga"]
