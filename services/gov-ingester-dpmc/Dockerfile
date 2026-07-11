FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-dpmc/ services/gov-dept-dpmc/
COPY services/gov-ingester-dpmc/ services/gov-ingester-dpmc/

# Build the Department of the Prime Minister and Cabinet ingester
RUN cargo build --release -p gov-ingester-dpmc

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-dpmc /usr/local/bin/gov-ingester-dpmc
COPY --from=builder /app/services/gov-ingester-dpmc/fixtures /fixtures

CMD ["gov-ingester-dpmc"]
