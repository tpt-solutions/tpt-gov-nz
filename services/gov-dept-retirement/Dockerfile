FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-retirement/ services/gov-dept-retirement/

# Build only the Retirement Commission (Te Ara Ahunga Ora) service
RUN cargo build --release -p gov-dept-retirement

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-retirement /usr/local/bin/gov-dept-retirement
COPY --from=builder /app/services/gov-dept-retirement/migrations /migrations

EXPOSE 8132 7052

CMD ["gov-dept-retirement"]
