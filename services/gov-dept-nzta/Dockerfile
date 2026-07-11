FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-nzta/ services/gov-dept-nzta/

# Build only the NZTA service
RUN cargo build --release -p gov-dept-nzta

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-nzta /usr/local/bin/gov-dept-nzta
COPY --from=builder /app/services/gov-dept-nzta/migrations /migrations

EXPOSE 8094 7005

CMD ["gov-dept-nzta"]
