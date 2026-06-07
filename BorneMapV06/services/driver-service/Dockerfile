# syntax=docker/dockerfile:1
FROM rust:1.95-slim-bookworm AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependency build by copying Cargo.toml files first
COPY Cargo.toml Cargo.lock ./
COPY services/driver-service/Cargo.toml services/driver-service/Cargo.toml
COPY crates/ev-core/Cargo.toml crates/ev-core/Cargo.toml
COPY crates/ev-geo/Cargo.toml crates/ev-geo/Cargo.toml
COPY crates/ev-db/Cargo.toml crates/ev-db/Cargo.toml

RUN mkdir -p services/driver-service/src \
    crates/ev-core/src \
    crates/ev-geo/src \
    crates/ev-db/src \
    && echo "fn main() {}" > services/driver-service/src/main.rs \
    && echo "" > crates/ev-core/src/lib.rs \
    && echo "" > crates/ev-geo/src/lib.rs \
    && echo "" > crates/ev-db/src/lib.rs

# Build dependencies with stubs so the layer is cached
RUN cargo build --release -p driver-service || true

# Now copy real source and rebuild
COPY . .

RUN cargo build --release -p driver-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/driver-service /usr/local/bin/driver-service
COPY db/migrations /app/db/migrations

EXPOSE 8000

HEALTHCHECK --interval=15s --timeout=5s --retries=3 --start-period=10s \
    CMD curl -f http://localhost:8000/health || exit 1

CMD ["driver-service"]
