FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-nzdf/ services/gov-dept-nzdf/
COPY services/gov-ingester-nzdf/ services/gov-ingester-nzdf/

# Build the New Zealand Defence Force ingester
RUN cargo build --release -p gov-ingester-nzdf

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-nzdf /usr/local/bin/gov-ingester-nzdf
COPY --from=builder /app/services/gov-ingester-nzdf/fixtures /fixtures

CMD ["gov-ingester-nzdf"]
