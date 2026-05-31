# Quickstart: Clickstream Tracking Pipeline

## Prerequisites

- Docker & Docker Compose (for RabbitMQ + MongoDB)
- Rust toolchain (for building analytics-service)
- Existing BorneMap stack (api-service, PostgreSQL)

## Step 1 — Start Infrastructure

```bash
cd deployments
docker compose up -d
```

This starts RabbitMQ (port 5672, management UI on 15672) and MongoDB (port 27017).

## Step 2 — Build and Run the Analytics Service

```bash
cd backend
cargo build -p analytics-service
DATABASE_URL="postgres://borne:borne@localhost:5432/borne_map" \
  cargo run -p analytics-service
```

The consumer connects to RabbitMQ and MongoDB, declares the `analytics.connections` queue, and begins processing events.

## Step 3 — Start the API Gateway

```bash
cd backend
DATABASE_URL="postgres://borne:borne@localhost:5432/borne_map" \
  cargo run -p api-service
```

The gateway now listens on `POST /api/v1/analytics/connect`.

## Step 4 — Verify the Pipeline

### Send a test event

```bash
curl -X POST http://localhost:8080/api/v1/analytics/connect \
  -H "Content-Type: application/json" \
  -d '{"event_id":"evt-f3a219b1","client_platform":"web","app_version":"1.14.0","connected_at":"2026-05-28T21:30:00Z"}'
```

Expected response: `202 Accepted`

### Query aggregates

```bash
curl http://localhost:8080/api/v1/analytics/connections
```

Expected response: list of platform aggregates with counts.

### Check consumer health

```bash
curl http://localhost:8181/health
```

Expected response: JSON with queue depth, last processed timestamp, and uptime.

### Audit aggregates directly in MongoDB

```bash
docker exec -it bornemap_nosql_dev mongosh -u admin -p secret_password_change_me --authenticationDatabase admin
```

Inside the MongoDB shell:

```javascript
use bornemap_analytics
db.connection_aggregates.find().pretty()
```

Expected response: per-platform documents with `total_connections_count`, `last_handshake_at`, and `engine_version`.

## Verification Checklist

- [ ] RabbitMQ management UI accessible at `http://localhost:15672` (guest/guest)
- [ ] MongoDB accessible at `localhost:27017` (admin/secret_password_change_me)
- [ ] POST /api/v1/analytics/connect returns 202
- [ ] GET /api/v1/analytics/connections returns aggregates
- [ ] GET /health returns consumer status
- [ ] App launches generate events without visible latency
