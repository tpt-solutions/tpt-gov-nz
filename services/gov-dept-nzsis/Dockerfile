FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-nzsis/ services/gov-dept-nzsis/

# Build only the New Zealand Security Intelligence Service service
RUN cargo build --release -p gov-dept-nzsis

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-nzsis /usr/local/bin/gov-dept-nzsis
COPY --from=builder /app/services/gov-dept-nzsis/migrations /migrations

EXPOSE 8147 7067

CMD ["gov-dept-nzsis"]
