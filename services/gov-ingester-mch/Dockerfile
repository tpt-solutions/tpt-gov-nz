FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mch/ services/gov-dept-mch/
COPY services/gov-ingester-mch/ services/gov-ingester-mch/

# Build the Ministry for Culture and Heritage ingester
RUN cargo build --release -p gov-ingester-mch

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-mch /usr/local/bin/gov-ingester-mch
COPY --from=builder /app/services/gov-ingester-mch/fixtures /fixtures

CMD ["gov-ingester-mch"]
