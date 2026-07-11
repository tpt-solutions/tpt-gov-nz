FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-treasury/ services/gov-dept-treasury/

# Build only the The Treasury service
RUN cargo build --release -p gov-dept-treasury

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-treasury /usr/local/bin/gov-dept-treasury
COPY --from=builder /app/services/gov-dept-treasury/migrations /migrations

EXPOSE 8120 7040

CMD ["gov-dept-treasury"]
