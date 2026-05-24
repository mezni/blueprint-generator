FROM rust:1.88-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p station-service

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /app/target/release/bornemap-backend /bornemap-backend
EXPOSE 8000
ENTRYPOINT ["/bornemap-backend"]
