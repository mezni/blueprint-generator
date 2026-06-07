FROM rust:1.87 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p analytics-writer

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/analytics-writer /usr/local/bin/analytics-writer
EXPOSE 8085
CMD ["analytics-writer"]
