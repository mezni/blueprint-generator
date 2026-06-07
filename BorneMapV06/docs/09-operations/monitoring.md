# Monitoring

## Health Checks

Each service exposes a `/health` endpoint:
- Returns `200 OK` when service is healthy
- Returns `503` when dependent services (DB, RabbitMQ) are unreachable

## Logging

- Structured JSON logging to stdout
- Log levels: error, warn, info, debug
- Each service logs request ID for tracing

## Metrics (Future)

- Consider Prometheus for metrics collection
- Consider Grafana for dashboard visualization
- Consider Loki for log aggregation
