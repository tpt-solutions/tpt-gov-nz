FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-moj/ services/gov-dept-moj/

# Build only the MOJ service
RUN cargo build --release -p gov-dept-moj

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-moj /usr/local/bin/gov-dept-moj
COPY --from=builder /app/services/gov-dept-moj/migrations /migrations

EXPOSE 8096 7007

CMD ["gov-dept-moj"]
