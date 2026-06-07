FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p driver-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/driver-service /usr/local/bin/driver-service
EXPOSE 8081
CMD ["driver-service"]
