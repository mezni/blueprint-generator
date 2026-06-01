# Communication Rules

## Purpose

Define how services communicate with each other and with frontend clients.
Synchronous (REST) and asynchronous (RabbitMQ) channels each have distinct
usage rules.

## Version

1.0.0

## Channel Matrix

| Origin → Destination | Channel | Protocol | Purpose |
|----------------------|---------|----------|---------|
| Frontend → Traefik | HTTP | TCP/80,443 | All external traffic |
| Traefik → Service | REST | HTTP/1.1 | Request routing |
| Driver Service → Clickstream | REST | HTTP | Event ingestion API |
| Clickstream Service → RMQ | AMQP | AMQP 0-9-1 | Event publishing |
| RMQ → Analytics Consumer | AMQP | AMQP 0-9-1 | Event consumption |
| Admin Service → RMQ | AMQP | AMQP 0-9-1 | GIS sync outbox events |
| RMQ → GIS Worker | AMQP | AMQP 0-9-1 | GIS sync events |
| Service → DB | Native | PostgreSQL wire | SQL queries |

## REST Rules

- Versioned endpoints: `/v1/<resource>`
- JSON request/response bodies
- Cursor-based pagination only (no offset/limit)
- Standard error format: `{"error_code", "message", "trace_id"}`
- Rate limited: public (strict), auth (stricter), ingestion (throttled)

## RabbitMQ Rules

- Exchange: topic-based
- Queue naming: `<service>.<event-type>`
- Consumer: at-least-once, auto-ack off
- Dead-letter: failed events routed to `<queue>.dlq`
- No business data in routing keys

## Forbidden Patterns

- Direct cross-service HTTP calls (always through Traefik)
- Cross-service DB reads or writes
- Shared in-memory state between service instances
- Synchronous event processing (no RPC-style RMQ request/reply)
