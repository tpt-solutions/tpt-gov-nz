FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY services/gov-dept-mpi/ services/gov-dept-mpi/
COPY services/gov-ingester-mpi/ services/gov-ingester-mpi/

# Build the MPI ingester
RUN cargo build --release -p gov-ingester-mpi

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gov-ingester-mpi /usr/local/bin/gov-ingester-mpi
COPY --from=builder /app/services/gov-ingester-mpi/fixtures /fixtures

CMD ["gov-ingester-mpi"]
