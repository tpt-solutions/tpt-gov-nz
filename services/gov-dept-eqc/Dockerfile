FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-eqc/ services/gov-dept-eqc/

# Build only the Earthquake Commission (Toka Tū Ake) service
RUN cargo build --release -p gov-dept-eqc

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-eqc /usr/local/bin/gov-dept-eqc
COPY --from=builder /app/services/gov-dept-eqc/migrations /migrations

EXPOSE 8134 7054

CMD ["gov-dept-eqc"]
