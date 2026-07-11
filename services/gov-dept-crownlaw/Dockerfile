FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-crownlaw/ services/gov-dept-crownlaw/

# Build only the Crown Law Office service
RUN cargo build --release -p gov-dept-crownlaw

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-crownlaw /usr/local/bin/gov-dept-crownlaw
COPY --from=builder /app/services/gov-dept-crownlaw/migrations /migrations

EXPOSE 8123 7043

CMD ["gov-dept-crownlaw"]
