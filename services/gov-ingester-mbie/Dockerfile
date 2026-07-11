FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mbie/ services/gov-dept-mbie/
COPY services/gov-ingester-mbie/ services/gov-ingester-mbie/

# Build the MBIE ingester
RUN cargo build --release -p gov-ingester-mbie

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-mbie /usr/local/bin/gov-ingester-mbie
COPY --from=builder /app/services/gov-ingester-mbie/fixtures /fixtures

CMD ["gov-ingester-mbie"]
