FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-nzdf/ services/gov-dept-nzdf/

# Build only the New Zealand Defence Force service
RUN cargo build --release -p gov-dept-nzdf

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-nzdf /usr/local/bin/gov-dept-nzdf
COPY --from=builder /app/services/gov-dept-nzdf/migrations /migrations

EXPOSE 8145 7065

CMD ["gov-dept-nzdf"]
