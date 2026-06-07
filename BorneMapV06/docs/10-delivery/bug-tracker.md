# Bug & Fix Tracker

> Central log of known bugs, issues, and their fixes across the project.

## Active

| ID | Date | Severity | Component | Description | Status |
|---|---|---|---|---|---|
| TBD | — | — | — | — | open |

## Resolved

| ID | Date | Severity | Component | Description | Fix |
|---|---|---|---|---|---|
| KL-001 | 2026-06-05 | medium | `crates/ev-db` | Pool tests cannot run without a live PostgreSQL instance | Mitigated: tests only check URL parsing, not pool creation |
| KL-002 | 2026-06-05 | high | Docker | Migration runner uses relative path `db/migrations/` not available in runtime image | Fixed: Dockerfile copies `db/migrations/` into runtime image at `/app/db/migrations`, WORKDIR set to `/app` |
| KL-005 | 2026-06-05 | medium | Docker | Compose healthcheck used fragile bash TCP hack instead of curl | Fixed: healthcheck now uses `curl -f http://localhost:8000/health` matching Dockerfile |

## Known Limitations

| ID | Component | Limitation | Impact | Planned Fix |
|---|---|---|---|---|
| KL-001 | `crates/ev-db` | Pool tests cannot run without a live PostgreSQL instance | `cargo test -p ev-db` may fail if no DB is available | Add testcontainers or mock in Sprint 1 |
| KL-002 | `services/driver-service` | Embedded migration runner uses relative path `db/migrations/` | Works in dev but requires migrations copied into Docker image for deployment | Address in Sprint 1 Docker optimization |
| KL-003 | Cargo workspace | `sqlx-postgres v0.7.4` emits future-incompat warning for upcoming Rust versions | Warning only, no functional impact | Upgrade to sqlx 0.8+ when stable |
| KL-004 | Frontend workspaces | `pnpm install` and dev server verification pending | Cannot verify frontend builds until pnpm 8+ is installed | Run `pnpm install` when tooling available |
| KL-005 | Docker stack | Full Docker compose up test requires Docker Engine | Cannot verify end-to-end until environment available | Run `docker compose up` when Docker available |

## Maintenance

- Last updated: 2026-06-05
- Update this file whenever a bug is found or a limitation is discovered.
- Use `open` → `in_progress` → `fixed` → `verified` lifecycle for bug status.
