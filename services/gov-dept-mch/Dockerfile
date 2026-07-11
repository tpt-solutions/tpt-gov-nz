FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mch/ services/gov-dept-mch/

# Build only the Ministry for Culture and Heritage service
RUN cargo build --release -p gov-dept-mch

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-mch /usr/local/bin/gov-dept-mch
COPY --from=builder /app/services/gov-dept-mch/migrations /migrations

EXPOSE 8142 7062

CMD ["gov-dept-mch"]
