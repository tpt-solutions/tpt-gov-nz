FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-fenz/ services/gov-dept-fenz/

# Build only the Fire and Emergency New Zealand service
RUN cargo build --release -p gov-dept-fenz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-fenz /usr/local/bin/gov-dept-fenz
COPY --from=builder /app/services/gov-dept-fenz/migrations /migrations

EXPOSE 8138 7058

CMD ["gov-dept-fenz"]
