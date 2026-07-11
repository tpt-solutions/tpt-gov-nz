FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-msd/ services/gov-dept-msd/

# Build only the MSD service
RUN cargo build --release -p gov-dept-msd

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-msd /usr/local/bin/gov-dept-msd
COPY --from=builder /app/services/gov-dept-msd/migrations /migrations

EXPOSE 8100 7011

CMD ["gov-dept-msd"]
