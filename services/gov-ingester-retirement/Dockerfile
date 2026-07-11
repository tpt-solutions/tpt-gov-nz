FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-retirement/ services/gov-dept-retirement/
COPY services/gov-ingester-retirement/ services/gov-ingester-retirement/

# Build the Retirement Commission (Te Ara Ahunga Ora) ingester
RUN cargo build --release -p gov-ingester-retirement

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-retirement /usr/local/bin/gov-ingester-retirement
COPY --from=builder /app/services/gov-ingester-retirement/fixtures /fixtures

CMD ["gov-ingester-retirement"]
