FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mbie/ services/gov-dept-mbie/

# Build only the MBIE service
RUN cargo build --release -p gov-dept-mbie

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-mbie /usr/local/bin/gov-dept-mbie
COPY --from=builder /app/services/gov-dept-mbie/migrations /migrations

EXPOSE 8101 7012

CMD ["gov-dept-mbie"]
