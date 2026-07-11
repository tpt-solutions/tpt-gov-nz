FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-worksafe/ services/gov-dept-worksafe/
COPY services/gov-ingester-worksafe/ services/gov-ingester-worksafe/

# Build the WorkSafe New Zealand ingester
RUN cargo build --release -p gov-ingester-worksafe

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-worksafe /usr/local/bin/gov-ingester-worksafe
COPY --from=builder /app/services/gov-ingester-worksafe/fixtures /fixtures

CMD ["gov-ingester-worksafe"]
