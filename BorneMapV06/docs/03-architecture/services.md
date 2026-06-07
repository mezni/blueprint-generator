# Services

## Keycloak (Identity Provider)

- Manages login, token issuance, session management
- Handles roles and authentication only
- Does NOT handle business data (favorites, reviews, stations, partner data)
- External system — not part of the Rust workspace

## Driver Service

Public discovery + registered driver features.

### Clean Architecture Mapping

| Layer | Modules | Description |
|-------|---------|-------------|
| Domain | `domain/station.rs`, `domain/favorite.rs`, `domain/review.rs` | Station, Favorite, Review entities; repository traits |
| Application | `application/stations.rs`, `application/reviews.rs`, `application/favorites.rs` | Nearby search, station detail, favorite toggle, review CRUD use cases |
| Infrastructure | `infrastructure/db/stations.rs`, `infrastructure/db/favorites.rs`, `infrastructure/db/reviews.rs` | SQLx queries implementing repository traits |
| Interface | `interface/handlers/stations.rs`, `interface/handlers/favorites.rs`, `interface/handlers/reviews.rs` | Actix-Web handlers; request parsing, response formatting |

### Shared Crates Used

- `ev-core` — NanoID, shared enums
- `ev-auth` — JWT validation, role checks
- `ev-db` — pool setup, pagination
- `ev-geo` — distance, bbox calculations

---

## Admin Service

Partner + admin management endpoints.

### Clean Architecture Mapping

| Layer | Modules | Description |
|-------|---------|-------------|
| Domain | `domain/partner.rs`, `domain/station.rs`, `domain/charger.rs`, `domain/availability.rs` | Partner, Station, Charger, Availability entities; repository traits |
| Application | `application/partners.rs`, `application/stations.rs`, `application/chargers.rs`, `application/availability.rs` | CRUD use cases with partner scope enforcement |
| Infrastructure | `infrastructure/db/partners.rs`, `infrastructure/db/stations.rs`, `infrastructure/db/chargers.rs`, `infrastructure/db/availability.rs`, `infrastructure/db/outbox.rs` | SQLx queries; outbox table writes for GIS sync |
| Interface | `interface/handlers/partners.rs`, `interface/handlers/stations.rs`, `interface/handlers/chargers.rs`, `interface/handlers/availability.rs`, `interface/handlers/moderation.rs`, `interface/handlers/reports.rs` | Actix-Web handlers; partner scope middleware |

### Partner Scope Enforcement

Partner scope is enforced at the **interface layer** via middleware and at the
**application layer** via scoped use cases. The pattern:

1. Middleware extracts `partner_id` from JWT claims
2. Application layer receives `partner_id` as a parameter
3. Infrastructure layer filters queries by `partner_id`

---

## Clickstream Service

Analytics event ingestion.

### Clean Architecture Mapping

| Layer | Modules | Description |
|-------|---------|-------------|
| Domain | `domain/event.rs` | Event type, validation rules |
| Application | (thin — validates + routes to publisher) | Direct domain → publisher flow |
| Infrastructure | `publisher/rabbitmq.rs` | RabbitMQ producer |
| Interface | `interface/handlers/ingest.rs` | HTTP endpoint, request validation |

### Notes

Clickstream Service is intentionally thin — events are validated at the
interface layer, modeled in domain, and published via infrastructure. No
complex application orchestration is needed.

---

## GIS Sync Worker

Background worker for asynchronous GIS updates.

### Clean Architecture Mapping

| Layer | Modules | Description |
|-------|---------|-------------|
| Domain | (none — pure sync logic is minimal) | — |
| Application | `processor/sync.rs`, `processor/resync.rs` | Outbox polling, station→GIS sync orchestration |
| Infrastructure | `db/outbox.rs`, `db/gis.rs`, `gis/artifacts.rs`, `gis/enrichment.rs` | Outbox reads, GIS writes, spatial enrichment |
| Interface | (none — no HTTP) | Runs as a background worker with no HTTP interface |

### Notes

The GIS Sync Worker has no interface layer (no HTTP). The application layer
polls an outbox table and calls infrastructure to sync spatial data.
