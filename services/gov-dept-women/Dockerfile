FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-women/ services/gov-dept-women/

# Build only the Ministry for Women service
RUN cargo build --release -p gov-dept-women

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-women /usr/local/bin/gov-dept-women
COPY --from=builder /app/services/gov-dept-women/migrations /migrations

EXPOSE 8126 7046

CMD ["gov-dept-women"]
