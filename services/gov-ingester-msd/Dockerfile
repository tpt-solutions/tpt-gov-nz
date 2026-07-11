FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-msd/ services/gov-dept-msd/
COPY services/gov-ingester-msd/ services/gov-ingester-msd/

# Build the MSD ingester
RUN cargo build --release -p gov-ingester-msd

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-msd /usr/local/bin/gov-ingester-msd
COPY --from=builder /app/services/gov-ingester-msd/fixtures /fixtures

CMD ["gov-ingester-msd"]
