FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-tearawhiti/ services/gov-dept-tearawhiti/

# Build only the Te Arawhiti service
RUN cargo build --release -p gov-dept-tearawhiti

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-tearawhiti /usr/local/bin/gov-dept-tearawhiti
COPY --from=builder /app/services/gov-dept-tearawhiti/migrations /migrations

EXPOSE 8129 7049

CMD ["gov-dept-tearawhiti"]
