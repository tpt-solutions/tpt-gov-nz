FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-dia/ services/gov-dept-dia/

# Build only the DIA service
RUN cargo build --release -p gov-dept-dia

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-dia /usr/local/bin/gov-dept-dia
COPY --from=builder /app/services/gov-dept-dia/migrations /migrations

EXPOSE 8093 7004

CMD ["gov-dept-dia"]
