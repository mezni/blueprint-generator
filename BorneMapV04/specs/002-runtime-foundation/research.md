# Research: Runtime Foundation

**Phase**: 0 | **Date**: 2026-06-01 | **Plan**: [plan.md](plan.md)

## Research Tasks

### 1. Config Loading Strategy

**Question**: How should Rust services load and validate environment configuration?

**Decision**: Use `serde` + manual env var parsing in a dedicated config module per service.

**Rationale**:
- Avoids pulling heavy framework dependencies (config-rs, envy, dotenvy) for a simple flat env-var structure
- Each service has <20 env vars — a typed struct with `From<HashMap<String, String>>` is sufficient
- Explicit validation (required fields, enum checks) is trivially implemented
- Consistent with the constitution's "fail-fast configuration" rule
- Startup prints a redacted summary per FR-008/FR-011

**Alternatives considered**:
- **config-rs**: Adds dependency weight, hierarchical config not needed for flat env vars
- **envy + serde**: Derive-based, but doesn't support validation errors beyond type mismatches
- **dotenvy**: Only loads `.env` files, doesn't parse or validate

### 2. PostgreSQL Connectivity (sqlx)

**Question**: Which PostgreSQL driver and connection pattern to use?

**Decision**: `sqlx` 0.8 with `tokio-postgres` runtime, connection pooling, explicit startup validation.

**Rationale**:
- sqlx is the de facto standard Rust PostgreSQL driver
- Built-in connection pooling (`PgPool`) handles reconnect with `test_on_acquire`
- `sqlx::PgPool::connect_with()` supports timeout via `PgConnectOptions`
- Startup validation: `SELECT 1` ping on pool creation
- Consistent with constitution's future DB migration requirements

**Alternatives considered**:
- **tokio-postgres** (raw): More control but requires manual pooling — unnecessary complexity
- **diesel**: ORM-weight for a simple connectivity layer — overkill for this sprint
- **sea-orm**: Async ORM, but too heavy for bootstrapping only

### 3. RabbitMQ Connectivity (lapin)

**Question**: Which AMQP client and connection pattern to use?

**Decision**: `lapin` 2.x with `ConnectionProperties` for retry and heartbeat.

**Rationale**:
- lapin is the most mature async Rust AMQP client
- `ConnectionProperties::default().with_heartbeat(10)` for liveness detection
- Startup: connect with timeout, declare queues, validate
- Runtime reconnect handled via `Connection::on_error` callback or simple reconnect wrapper

**Alternatives considered**:
- **amiquip**: Synchronous only, doesn't fit tokio async model
- **deadpool-lapin**: Connection pool wrapper — useful later, but single-connection is fine for this scale

### 4. Structured Logging (tracing)

**Question**: Which logging framework produces structured JSON logs?

**Decision**: `tracing` 0.1 + `tracing-subscriber` with `json` formatter.

**Rationale**:
- tracing is the standard async-aware diagnostics framework in Rust
- `tracing-subscriber::fmt().json()` produces structured JSON output with zero configuration
- `tracing` spans map naturally to request correlation (future use)
- `service.name` and `environment` injected via `FmtSubscriber`'s `with_env_filter`

**Alternatives considered**:
- **log + slog**: Structured but lacks async span support — tracing is now standard
- **flexi_logger**: File-focused, not suitable for container stdout logging
- **env_logger**: Simple but no JSON output

### 5. Health/Readiness Probe Pattern

**Question**: How should /health and /ready endpoints work?

**Decision**: `/health` is a static OK response; `/ready` checks all declared dependencies.

**Rationale**:
- `/health`: Responds immediately with 200 + service metadata — used by load balancers / orchestration for liveness
- `/ready`: Iterates through registered dependency checks (DB ping, RabbitMQ connection state) — returns 200 if all pass, 503 with details if any fail
- Dependency checks cached briefly (5s TTL) to avoid hammering dependencies on every probe
- Pattern follows Kubernetes readiness probe semantics

### 6. Docker Compose Health Checks

**Question**: How to implement container health checks?

**Decision**: Use Docker Compose `healthcheck` with `interval`, `timeout`, `retries`, and `start_period`.

**Infrastructure health checks**:
- **PostgreSQL**: `pg_isready -U <user> -d <db>`
- **RabbitMQ**: `rabbitmq-diagnostics -q ping`
- **Keycloak**: curl to `{KEYCLOAK_PUBLIC_URL}/realms/ev-platform` (HTTP 200 check)
- **Traefik**: Built-in health endpoint at `:8080/api/http/routers`

**Service health checks**: curl to each service's `/health` endpoint

### 7. Environment Profile Strategy

**Question**: How to implement `local`, `docker`, and `staging` profiles?

**Decision**: Shell-sourced `.env` files per profile under `infra/env/{profile}/`.

- `APP_ENV` variable selects the profile
- Docker Compose uses `env_file` directives pointing to the profile-specific env directory
- A `.env.example` at the root documents all variables
- `local` profile: relaxed validation, host-mapped ports for debugging
- `docker` profile: internal networking only, no host port exposure for services

### 8. Keycloak Realm Import

**Question**: How to provision the `ev-platform` realm and clients?

**Decision**: Export realm JSON and mount it as a volume for Keycloak's automatic import (`KEYCLOAK_IMPORT`).

- Realm JSON exported from a working Keycloak instance as a starting point
- Includes `ev-platform` realm with placeholder OIDC clients
- Clients defined with dummy redirect URIs (`http://localhost:*/callback`)
- Import is idempotent — Keycloak skips existing realms on re-import

### 9. RabbitMQ Queue Declaration

**Question**: How to ensure queues exist on boot?

**Decision**: Use a RabbitMQ init container with `rabbitmqadmin` or a shell script.

- Alternative A: `.rabbitmq/definitions.json` via `load_definitions` feature
- Alternative B: Docker entrypoint script using `rabbitmqadmin declare queue`
- **Chosen**: Definitions file at `/etc/rabbitmq/definitions/` mounted as volume, loaded via `RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS` or `load_definitions`

### 10. Smoke Test Tooling

**Question**: What tooling for the smoke test script?

**Decision**: Pure bash + curl + jq.

- No language dependencies beyond coreutils, curl, jq
- Tests: DB ping, RMQ queue list, Keycloak realm fetch, Traefik route list, service /health and /ready
- Exit code 0 if all pass, non-zero with details on first failure
- `scripts/smoke-test.sh` at repo root

---

## Summary of Decisions

| Category | Decision | Rationale |
|----------|----------|-----------|
| Config loader | serde + manual parse | Minimal deps, typed structs, explicit validation |
| DB driver | sqlx 0.8 + PgPool | Standard choice, built-in pooling + reconnect |
| RMQ client | lapin 2.x | Mature async AMQP client |
| Logging | tracing + json | Standard async diagnostics, JSON output built-in |
| Health check | static liveness, dependency-aware readiness | Follows k8s probe semantics |
| Env profiles | per-profile .env dirs | Clean separation, no code changes |
| Realm import | Keycloak JSON volume mount | Zero-config, idempotent |
| Smoke test | bash + curl + jq | No language toolchain required |
