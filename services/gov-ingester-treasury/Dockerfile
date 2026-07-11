FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-treasury/ services/gov-dept-treasury/
COPY services/gov-ingester-treasury/ services/gov-ingester-treasury/

# Build the The Treasury ingester
RUN cargo build --release -p gov-ingester-treasury

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-treasury /usr/local/bin/gov-ingester-treasury
COPY --from=builder /app/services/gov-ingester-treasury/fixtures /fixtures

CMD ["gov-ingester-treasury"]
