# Workspace Conventions

## Naming Rules

- **Services**: kebab-case (`driver-service`, `admin-service`)
- **Crates**: kebab-case (`common-types`, `common-errors`)
- **Apps**: kebab-case (`driver-web`, `partner-dashboard`)
- **Packages**: kebab-case (`shared-types`, `api-client`)
- **Rust crates**: Cargo package name matches directory name
- **TypeScript packages**: npm package name matches directory name

## Ownership Boundaries

- `services/` — Backend Rust service crates (one per bounded context)
- `crates/` — Shared Rust library crates (no binary targets)
- `apps/` — Frontend applications (web + mobile)
- `packages/` — Shared TypeScript packages
- `infra/` — Infrastructure configuration (Docker, env)
- `docs/` — Architecture and design documentation

## Commit Conventions

- Use conventional commits: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`
- Commit after each logical task group
- Do not commit build artifacts or `node_modules`
