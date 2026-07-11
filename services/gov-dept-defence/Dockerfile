FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-defence/ services/gov-dept-defence/

# Build only the Ministry of Defence service
RUN cargo build --release -p gov-dept-defence

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-defence /usr/local/bin/gov-dept-defence
COPY --from=builder /app/services/gov-dept-defence/migrations /migrations

EXPOSE 8144 7064

CMD ["gov-dept-defence"]
