FROM rust:1.87 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p clickstream-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/clickstream-service /usr/local/bin/clickstream-service
EXPOSE 8083
CMD ["clickstream-service"]
