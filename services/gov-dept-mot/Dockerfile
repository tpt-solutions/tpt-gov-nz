FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mot/ services/gov-dept-mot/

# Build only the Ministry of Transport service
RUN cargo build --release -p gov-dept-mot

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-mot /usr/local/bin/gov-dept-mot
COPY --from=builder /app/services/gov-dept-mot/migrations /migrations

EXPOSE 8135 7055

CMD ["gov-dept-mot"]
