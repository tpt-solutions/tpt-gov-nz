FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-eqc/ services/gov-dept-eqc/
COPY services/gov-ingester-eqc/ services/gov-ingester-eqc/

# Build the Earthquake Commission (Toka Tū Ake) ingester
RUN cargo build --release -p gov-ingester-eqc

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-eqc /usr/local/bin/gov-ingester-eqc
COPY --from=builder /app/services/gov-ingester-eqc/fixtures /fixtures

CMD ["gov-ingester-eqc"]
