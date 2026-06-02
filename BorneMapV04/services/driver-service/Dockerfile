FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY services ./services

RUN cargo build --release --package driver-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/driver-service /usr/local/bin/driver-service
CMD ["driver-service"]
