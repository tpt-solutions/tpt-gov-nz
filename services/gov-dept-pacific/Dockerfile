FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-pacific/ services/gov-dept-pacific/

# Build only the Ministry for Pacific Peoples service
RUN cargo build --release -p gov-dept-pacific

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-pacific /usr/local/bin/gov-dept-pacific
COPY --from=builder /app/services/gov-dept-pacific/migrations /migrations

EXPOSE 8127 7047

CMD ["gov-dept-pacific"]
