FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-dpmc/ services/gov-dept-dpmc/

# Build only the Department of the Prime Minister and Cabinet service
RUN cargo build --release -p gov-dept-dpmc

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-dpmc /usr/local/bin/gov-dept-dpmc
COPY --from=builder /app/services/gov-dept-dpmc/migrations /migrations

EXPOSE 8121 7041

CMD ["gov-dept-dpmc"]
