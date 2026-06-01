# Quickstart: Architecture Contracts

## Purpose

This quickstart helps stakeholders (architects, developers, reviewers)
navigate the contract documents produced by this feature and understand
how they relate to the platform constitution.

## Prerequisites

- Read `docs/constitution.md` (platform constitution v5.1)
- Read `docs/epic00.md` (EPIC 0 system constitution breakdown)

## Contract Documents

The 8 contract documents are in `specs/001-architecture-contracts/contracts/`:

| # | Document | Read this if you need to... |
|---|----------|---------------------------|
| 1 | `architecture-contract.md` | Understand service boundaries and what each service owns |
| 2 | `service-matrix.md` | Look up which service owns which tables |
| 3 | `event-spec-v1.md` | Know what clickstream events to emit or consume |
| 4 | `rbac-model.md` | Understand authorization rules and role enforcement |
| 5 | `id-strategy.md` | Generate or validate entity IDs |
| 6 | `communication-rules.md` | Learn how services talk to each other |
| 7 | `ci-cd-contract.md` | Set up CI/CD pipelines |
| 8 | `database-schema-contract.md` | Write migrations or design queries |

## Suggested Reading Order

1. `architecture-contract.md` — Overall system structure
2. `database-schema-contract.md` — Data backbone
3. `service-matrix.md` — Who owns what
4. `communication-rules.md` — How they talk
5. `rbac-model.md` — Who can do what
6. `id-strategy.md` — How things are identified
7. `event-spec-v1.md` — Analytics events
8. `ci-cd-contract.md` — Build and deploy

## Next Steps

After reviewing these contracts, proceed to the first implementation EPIC:

1. **EPIC 1**: Monorepo + workspace bootstrap
2. **EPIC 4**: Keycloak integration (identity)
3. **EPIC 5**: PostgreSQL schema migrations
4. **EPIC 6**: Admin Service (inventory CRUD)

Each implementation EPIC must conform to the contracts defined here.
