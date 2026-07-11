FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-statsnz/ services/gov-dept-statsnz/
COPY services/gov-ingester-statsnz/ services/gov-ingester-statsnz/

# Build the Stats NZ ingester
RUN cargo build --release -p gov-ingester-statsnz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-statsnz /usr/local/bin/gov-ingester-statsnz
COPY --from=builder /app/services/gov-ingester-statsnz/fixtures /fixtures

CMD ["gov-ingester-statsnz"]
