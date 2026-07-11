FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-tec/ services/gov-dept-tec/

# Build only the Tertiary Education Commission service
RUN cargo build --release -p gov-dept-tec

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-tec /usr/local/bin/gov-dept-tec
COPY --from=builder /app/services/gov-dept-tec/migrations /migrations

EXPOSE 8141 7061

CMD ["gov-dept-tec"]
