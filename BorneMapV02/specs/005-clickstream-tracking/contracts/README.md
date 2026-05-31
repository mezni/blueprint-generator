# Contracts: Clickstream Tracking Pipeline

## Interface Contracts

| Contract | From | To | Protocol | File |
|----------|------|----|----------|------|
| Connection Event Ingestion | mobile-driver | api-service | HTTP REST | [ingestion-api.md](./ingestion-api.md) |
| Aggregates Query | admin/monitoring | api-service | HTTP REST | [aggregates-api.md](./aggregates-api.md) |
| Event Queue | api-service | analytics-service | AMQP 0-9-1 | [event-queue.md](./event-queue.md) |
| Consumer Health | monitoring | analytics-service | HTTP | [health-endpoint.md](./health-endpoint.md) |
