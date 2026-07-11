FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-nzqa/ services/gov-dept-nzqa/
COPY services/gov-ingester-nzqa/ services/gov-ingester-nzqa/

# Build the NZQA ingester
RUN cargo build --release -p gov-ingester-nzqa

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-nzqa /usr/local/bin/gov-ingester-nzqa
COPY --from=builder /app/services/gov-ingester-nzqa/fixtures /fixtures

CMD ["gov-ingester-nzqa"]
