FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-statsnz/ services/gov-dept-statsnz/

# Build only the Stats NZ service
RUN cargo build --release -p gov-dept-statsnz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-statsnz /usr/local/bin/gov-dept-statsnz
COPY --from=builder /app/services/gov-dept-statsnz/migrations /migrations

EXPOSE 8103 7014

CMD ["gov-dept-statsnz"]
