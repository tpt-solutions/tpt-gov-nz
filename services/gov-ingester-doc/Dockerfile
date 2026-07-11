FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-doc/ services/gov-dept-doc/
COPY services/gov-ingester-doc/ services/gov-ingester-doc/

# Build the DOC ingester
RUN cargo build --release -p gov-ingester-doc

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-doc /usr/local/bin/gov-ingester-doc
COPY --from=builder /app/services/gov-ingester-doc/fixtures /fixtures

CMD ["gov-ingester-doc"]
