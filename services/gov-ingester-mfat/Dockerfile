FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mfat/ services/gov-dept-mfat/
COPY services/gov-ingester-mfat/ services/gov-ingester-mfat/

# Build the Ministry of Foreign Affairs and Trade ingester
RUN cargo build --release -p gov-ingester-mfat

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-mfat /usr/local/bin/gov-ingester-mfat
COPY --from=builder /app/services/gov-ingester-mfat/fixtures /fixtures

CMD ["gov-ingester-mfat"]
