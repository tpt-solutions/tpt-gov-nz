FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-tearawhiti/ services/gov-dept-tearawhiti/
COPY services/gov-ingester-tearawhiti/ services/gov-ingester-tearawhiti/

# Build the Te Arawhiti ingester
RUN cargo build --release -p gov-ingester-tearawhiti

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-tearawhiti /usr/local/bin/gov-ingester-tearawhiti
COPY --from=builder /app/services/gov-ingester-tearawhiti/fixtures /fixtures

CMD ["gov-ingester-tearawhiti"]
