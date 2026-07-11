FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mfat/ services/gov-dept-mfat/

# Build only the Ministry of Foreign Affairs and Trade service
RUN cargo build --release -p gov-dept-mfat

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-mfat /usr/local/bin/gov-dept-mfat
COPY --from=builder /app/services/gov-dept-mfat/migrations /migrations

EXPOSE 8143 7063

CMD ["gov-dept-mfat"]
