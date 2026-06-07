# Clean Architecture

Every Rust backend service MUST follow Clean Architecture with four strictly
separated layers.

## Layer Structure

```
┌──────────────────────────────────────┐
│        Interface Layer (HTTP)         │
│   handlers/ — request parsing,       │
│   response formatting, middleware     │
│   Depends on: application layer      │
├──────────────────────────────────────┤
│      Application Layer (Use Cases)   │
│   application/ — orchestration,      │
│   service logic, transaction scoping │
│   Depends on: domain layer           │
├──────────────────────────────────────┤
│    Infrastructure Layer (External)    │
│   db/ — SQLx queries, repositories   │
│   Depends on: domain layer           │
├──────────────────────────────────────┤
│          Domain Layer (Pure)          │
│   domain/ — entities, value objects, │
│   enums, traits, business rules      │
│   Zero external dependencies         │
└──────────────────────────────────────┘
```

## Rust Directory Layout

Each service follows this module structure under `src/`:

```
services/<service>/src/
  main.rs              # Binary entrypoint
  config.rs            # Environment config
  router.rs            # Actix-Web route definitions
  domain/              # ── Domain Layer ──
    mod.rs
    station.rs         # Station entity, StationSummary, StationDetail
    partner.rs         # Partner entity (where applicable)
    review.rs          # Review entity (where applicable)
    favorite.rs        # Favorite entity (where applicable)
    events.rs          # Domain events (where applicable)
  application/         # ── Application Layer ──
    mod.rs
    stations.rs        # Station use cases (nearby, search, detail)
    reviews.rs         # Review use cases
    favorites.rs       # Favorite use cases
  infrastructure/      # ── Infrastructure Layer ──
    mod.rs
    db/
      mod.rs
      pool.rs          # SQLx PgPool setup
      stations.rs      # Station queries
      reviews.rs       # Review queries
      favorites.rs     # Favorite queries
  interface/           # ── Interface Layer ──
    mod.rs
    handlers/
      mod.rs
      stations.rs      # HTTP handlers for station endpoints
      reviews.rs       # HTTP handlers for review endpoints
      favorites.rs     # HTTP handlers for favorite endpoints
    middleware/
      mod.rs
      auth.rs          # JWT validation middleware
      logging.rs       # Request logging middleware
  errors.rs            # Typed errors → HTTP status mapping
```

## Shared Crates (Workspace-Level Domain & Infrastructure)

Domain logic shared across services lives in dedicated workspace crates:

| Crate | Layer | Purpose |
|-------|-------|---------|
| `crates/ev-core` | Domain | NanoID generation, shared enums, value objects |
| `crates/ev-auth` | Domain | JWT claims, role types, token validation traits |
| `crates/ev-db` | Infrastructure | PgPool setup, pagination primitives |
| `crates/ev-geo` | Domain | LatLng, bounding box, distance calculation |

## Dependency Rules

```
interface/  ──depends on──▶  application/  ──depends on──▶  domain/
                                  │
                                  └──depends on──▶  infrastructure/  ──depends on──▶  domain/
```

1. **Domain layer** — zero external framework dependencies. Only pure Rust and
   workspace crates (`ev-core`, `ev-auth`). No Actix-Web, no SQLx, no serde
   (except where required for Serialize/Deserialize on domain types).
2. **Application layer** — depends on domain layer only. Orchestrates use
   cases. Accepts trait implementations from infrastructure via dependency
   injection.
3. **Infrastructure layer** — implements traits defined by the application or
   domain layer. Concrete DB queries, HTTP clients, message producers.
4. **Interface layer** — depends on application layer. Converts HTTP requests
   to application use case calls, formats responses. Contains no business
   logic.

## Trait-Based Dependency Inversion

The application layer defines repository traits; infrastructure implements
them:

```rust
// domain/station.rs — trait definition (zero deps)
pub trait StationRepository {
    fn find_nearby(&self, lat: f64, lng: f64, radius_km: f64)
        -> Result<Vec<StationSummary>, Error>;
    fn find_by_id(&self, id: &str) -> Result<StationDetail, Error>;
}

// application/stations.rs — use case depends on trait
pub fn get_nearby_stations(
    repo: &impl StationRepository,
    lat: f64, lng: f64, radius_km: f64,
) -> Result<Vec<StationSummary>, Error> {
    repo.find_nearby(lat, lng, radius_km)
}

// infrastructure/db/stations.rs — concrete implementation
impl StationRepository for PgStationRepo {
    fn find_nearby(/*...*/) -> Result<Vec<StationSummary>, Error> {
        sqlx::query_as!(/*...*/).fetch_all(&self.pool).await
    }
}
```

## Error Handling

Each service has a single `errors.rs` that defines typed errors and maps them
to HTTP status codes at the interface layer:

```rust
// errors.rs (lives at service root, not in any specific layer)
pub enum ServiceError {
    NotFound(String),
    Unauthorized,
    ValidationError(String),
    Internal(String),
}

// interface/handlers/mod.rs — conversion to HTTP response
impl HttpResponse for ServiceError { /* map to Actix-Web response */ }
```

## Testing by Layer

| Layer | Test Type | Location |
|-------|-----------|----------|
| Domain | Unit tests | `src/domain/*.rs` (inline `#[cfg(test)]`) |
| Application | Unit + integration | `src/application/*.rs` + `tests/` with mocked repos |
| Infrastructure | Integration | `tests/integration/` with test database |
| Interface | Integration | `tests/integration/` — full HTTP request/response |
