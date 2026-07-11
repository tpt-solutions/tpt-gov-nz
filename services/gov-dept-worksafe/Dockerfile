FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-worksafe/ services/gov-dept-worksafe/

# Build only the WorkSafe New Zealand service
RUN cargo build --release -p gov-dept-worksafe

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-dept-worksafe /usr/local/bin/gov-dept-worksafe
COPY --from=builder /app/services/gov-dept-worksafe/migrations /migrations

EXPOSE 8131 7051

CMD ["gov-dept-worksafe"]
