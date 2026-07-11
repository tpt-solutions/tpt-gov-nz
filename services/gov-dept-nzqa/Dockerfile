FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-nzqa/ services/gov-dept-nzqa/

# Build only the NZQA service
RUN cargo build --release -p gov-dept-nzqa

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-nzqa /usr/local/bin/gov-dept-nzqa
COPY --from=builder /app/services/gov-dept-nzqa/migrations /migrations

EXPOSE 8099 7010

CMD ["gov-dept-nzqa"]
