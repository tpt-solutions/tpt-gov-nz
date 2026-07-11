FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-ethnic/ services/gov-dept-ethnic/

# Build only the Ministry for Ethnic Communities service
RUN cargo build --release -p gov-dept-ethnic

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-ethnic /usr/local/bin/gov-dept-ethnic
COPY --from=builder /app/services/gov-dept-ethnic/migrations /migrations

EXPOSE 8128 7048

CMD ["gov-dept-ethnic"]
