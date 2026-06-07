# Testing Strategy

## Test Levels

### Unit Tests
- Rust: `cargo test` for service unit tests
- Frontend: Vitest for component and hook tests
- Located alongside source code

### Integration Tests
- Located in `tests/integration/` per service
- Test full API endpoints with test database
- Cover: station CRUD, favorites, reviews, auth, scope enforcement

### Contract Tests
- Verify API contracts between services
- Ensure backward compatibility

## Test Targets

| Service | Test Command | Location |
|---------|-------------|----------|
| Driver Service | `cargo test -p driver-service` | `services/driver-service/tests/` |
| Admin Service | `cargo test -p admin-service` | `services/admin-service/tests/` |
| Clickstream Service | `cargo test -p clickstream-service` | `services/clickstream-service/tests/` |
| GIS Sync Worker | `cargo test -p gis-sync-worker` | `services/gis-sync-worker/tests/` |
| UI Components | `pnpm --filter @bornemap/ui test` | `packages/ui/` |
| Frontend Apps | `pnpm --filter <app> test` | `apps/*/` |

## CI Pipeline

GitHub Actions runs:
1. `cargo test` — all Rust workspace tests
2. `pnpm lint` — lint all frontend packages
3. `pnpm test` — all frontend tests

## Key Integration Test Scenarios

- Partner scope enforcement (partner cannot access another org's data)
- Public access without authentication
- JWT validation failures
- Station soft delete hiding from public
- Review moderation flow
