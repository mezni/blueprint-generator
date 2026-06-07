FROM rust:1.87 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p gis-worker

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/gis-worker /usr/local/bin/gis-worker
EXPOSE 8084
CMD ["gis-worker"]
