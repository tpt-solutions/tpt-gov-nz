FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-moe/ services/gov-dept-moe/

# Build only the Ministry of Education service
RUN cargo build --release -p gov-dept-moe

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-moe /usr/local/bin/gov-dept-moe
COPY --from=builder /app/services/gov-dept-moe/migrations /migrations

EXPOSE 8139 7059

CMD ["gov-dept-moe"]
