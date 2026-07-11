FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mfe/ services/gov-dept-mfe/

# Build only the Ministry for the Environment service
RUN cargo build --release -p gov-dept-mfe

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-mfe /usr/local/bin/gov-dept-mfe
COPY --from=builder /app/services/gov-dept-mfe/migrations /migrations

EXPOSE 8133 7053

CMD ["gov-dept-mfe"]
