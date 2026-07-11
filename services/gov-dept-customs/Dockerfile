FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-customs/ services/gov-dept-customs/

# Build only the Customs service
RUN cargo build --release -p gov-dept-customs

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-customs /usr/local/bin/gov-dept-customs
COPY --from=builder /app/services/gov-dept-customs/migrations /migrations

EXPOSE 8105 7016

CMD ["gov-dept-customs"]
