# Tasks: Architecture Contracts

**Input**: Design documents from `specs/001-architecture-contracts/`

**Prerequisites**: plan.md (required), spec.md, research.md, data-model.md, contracts/

**Note**: This feature produces 8 contract documents defining the platform's
system constitution. The contracts have been drafted during the plan phase;
tasks below focus on review, validation, and finalization.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to
- Include exact file paths in descriptions

## Path Conventions

- Contracts: `specs/001-architecture-contracts/contracts/<name>.md`
- Review outputs: inline within contract docs

---

## Phase 1: Setup

**Purpose**: Ensure feature context is loaded and review framework is ready

- [X] T001 Read platform constitution at `docs/constitution.md` and `docs/epic00.md` to establish full context
- [X] T002 [P] Read all existing draft contracts under `specs/001-architecture-contracts/contracts/`
- [X] T003 [P] Read clarified specification at `specs/001-architecture-contracts/spec.md`
- [X] T004 [P] Read research decisions at `specs/001-architecture-contracts/research.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Review conventions and architectural invariants that all contracts must satisfy

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Review the architectural invariant in `docs/constitution.md` section 23
      (`inventory.station` is source of truth) and ensure it is enforced in
      every contract document
- [X] T006 [P] Review the data ownership matrix in
      `specs/001-architecture-contracts/contracts/service-matrix.md` for
      completeness and verify no ownership overlaps exist
- [X] T007 [P] Review the communication rules in
      `specs/001-architecture-contracts/contracts/communication-rules.md` to
      confirm cross-service DB access is forbidden everywhere

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel

---

## Phase 3: User Story 1 — Define Service Architecture Boundaries (Priority: P1) 🎯 MVP

**Goal**: Service boundaries, communication rules, and data ownership finalized
so that all teams build against the same contract.

**Independent Test**: A reviewer can verify that each service has a clearly
documented responsibility, a list of owned DB tables, and that no two services
claim ownership of the same data write path.

### Review Existing Contracts

- [X] T008 [P] [US1] Review `specs/001-architecture-contracts/contracts/architecture-contract.md`
      — verified: all 6 services, correct roles, DB access, public-facing status, invariant, enforcement
- [X] T009 [P] [US1] Review `specs/001-architecture-contracts/contracts/service-matrix.md`
      — verified: all services mapped, owned tables listed, no ownership overlaps, partner scoping
- [X] T010 [P] [US1] Review `specs/001-architecture-contracts/contracts/communication-rules.md`
      — verified: channel matrix complete, REST/RMQ rules, forbidden patterns explicit
- [X] T011 [P] [US1] Review `specs/001-architecture-contracts/contracts/id-strategy.md`
      — verified: all 5 prefixes documented, format rules, consistency + enforcement sections

### Finalize Contracts

- [X] T012 [US1] Update `specs/001-architecture-contracts/contracts/architecture-contract.md`
      — verified: no corrections needed (enforcement section present)
- [X] T013 [US1] Update `specs/001-architecture-contracts/contracts/service-matrix.md`
      — verified: no corrections needed
- [X] T014 [US1] Update `specs/001-architecture-contracts/contracts/communication-rules.md`
      — verified: no corrections needed
- [X] T015 [US1] Update `specs/001-architecture-contracts/contracts/id-strategy.md`
      — verified: no corrections needed

**Checkpoint**: At this point, User Story 1 should be fully functional and
testable independently — all architecture boundary contracts finalized.

---

## Phase 4: User Story 2 — Define PostgreSQL Schema Contracts (Priority: P1)

**Goal**: All four schemas (`inventory`, `users`, `gis`, `analytics`) fully
specified so that migrations can be written unambiguously.

**Independent Test**: Each schema spec can be reviewed independently for table
lists, column rules, constraint definitions, and ownership.

### Review Existing Contract

- [X] T016 [P] [US2] Review `specs/001-architecture-contracts/contracts/database-schema-contract.md`
      — verified: all 4 schemas, correct tables, columns, PK/FK/UNIQUE/GIST constraints,
      partitioning rules, soft delete columns, NanoID PKs, composite PKs

### Finalize Contract

- [X] T017 [US2] Update `specs/001-architecture-contracts/contracts/database-schema-contract.md`
      — verified: no corrections needed

**Checkpoint**: At this point, User Stories 1 AND 2 should both work
independently — architecture and database contracts finalized.

---

## Phase 5: User Story 3 — Define Clickstream Event Contract (Priority: P2)

**Goal**: Event envelope and event type list finalized so that frontend teams
and analytics consumers build against the same schema.

**Independent Test**: A mock event producer can send a valid envelope and a
mock consumer can parse it without schema negotiation.

### Review Existing Contract

- [X] T018 [P] [US3] Review `specs/001-architecture-contracts/contracts/event-spec-v1.md`
      — verified: event envelope with all 7 fields, all 9 event types with payload fields,
      delivery rules (at-least-once, no secrets, JSONB-only)

### Finalize Contract

- [X] T019 [US3] Update `specs/001-architecture-contracts/contracts/event-spec-v1.md`
      — verified: no corrections needed

**Checkpoint**: At this point, User Stories 1–3 should all work independently
— architecture, database, and event contracts finalized.

---

## Phase 6: User Story 4 — Define RBAC Model in Keycloak (Priority: P2)

**Goal**: Role model and enforcement layers defined so that authentication and
authorization are consistent across all services.

**Independent Test**: A test can validate that each of the three roles exists
in the model and that enforcement is specified at three layers: Keycloak,
service layer, and DB constraints.

### Review Existing Contract

- [X] T020 [P] [US4] Review `specs/001-architecture-contracts/contracts/rbac-model.md`
      — verified: exactly 3 roles, 3 enforcement layers (Keycloak/service/DB),
      partner isolation rule at repository level with no API-layer exceptions

### Finalize Contract

- [X] T021 [US4] Update `specs/001-architecture-contracts/contracts/rbac-model.md`
      — verified: no corrections needed

**Checkpoint**: At this point, User Stories 1–4 should all work independently.

---

## Phase 7: User Story 5 — Define CI/CD and Observability Contracts (Priority: P3)

**Goal**: CI/CD pipeline, observability standards, caching strategy, and
security rules finalized so that the operational foundation is locked.

**Independent Test**: A reviewer can verify that the pipeline stages, logging
format, metric list, and cache invalidation rules are documented and unambiguous.

### Review Existing Contract

- [X] T022 [P] [US5] Review `specs/001-architecture-contracts/contracts/ci-cd-contract.md`
      — verified: all 6 pipeline stages (lint/test/build/contract/Docker/GHCR),
      backend + frontend build rules, artifact tagging (git-sha), security rules

### Finalize Contract

- [X] T023 [US5] Update `specs/001-architecture-contracts/contracts/ci-cd-contract.md`
      — verified: no corrections needed (no-auto-deploy and manual deploy documented)

**Checkpoint**: At this point, all 5 user stories should be independently
functional — all 8 contract documents finalized and reviewed.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, consistency pass, and downstream integration

- [X] T024 [P] Run a cross-document consistency check — verified: no contradicting
      rules across all 8 contract documents (service counts, ownership, schemas,
      roles, IDs, events all consistent)
- [X] T025 [P] Verify the quickstart guide at
      `specs/001-architecture-contracts/quickstart.md` — verified: all 8 contracts
      referenced with correct file paths and reading order
- [X] T026 [P] Verify that `AGENTS.md` references the plan file at
      `specs/001-architecture-contracts/plan.md` and contracts directory
- [X] T027 [P] Verify every contract document has a Version line, Purpose
      statement, and Enforcement section — verified: all 8 contracts complete
- [X] T028 Update `docs/tasks.md` to reflect that EPIC 0 (Architecture Freeze)
      contract documents are now finalized and ready to unblock downstream EPICs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3–7)**: All depend on Foundational phase completion
  - User stories CAN proceed in parallel if staffed
  - Or sequentially in priority order (P1 US1 → P1 US2 → P2 US3 → P2 US4 → P3 US5)

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — locks architecture that downstream stories reference
- **US2 (P1)**: Can start after Phase 2 — locks DB schema that downstream stories reference
- **US3 (P2)**: Can start after Phase 2 — standalone event contract, no US dependency
- **US4 (P2)**: Can start after Phase 2 — standalone RBAC contract, no US dependency
- **US5 (P3)**: Can start after Phase 2 — may reference other contracts for CI pipeline design

### Within Each User Story

- Review before finalize
- Independent contracts (no cross-file editing conflicts)

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel
- All user story review tasks are [P] — independent contract files
- Finalize tasks are sequential per story but stories can run in parallel across team

---

## Parallel Example: User Story 1

```bash
# Launch all reviews for User Story 1 together:
Task: "Review architecture-contract.md"
Task: "Review service-matrix.md"
Task: "Review communication-rules.md"
Task: "Review id-strategy.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (architecture boundaries) → **MVP achieved**
4. STOP and VALIDATE: Review architecture-contract.md, service-matrix.md,
   communication-rules.md, id-strategy.md are all finalized
5. Downstream EPICs (Admin Service, GIS Worker, etc.) can begin against these contracts

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add US1 (Architecture) → Validate independently → **MVP!**
3. Add US2 (DB Schema) → Validate independently → Data contracts locked
4. Add US3 (Events) → Validate independently → Event contracts locked
5. Add US4 (RBAC) → Validate independently → Security contracts locked
6. Add US5 (CI/CD) → Validate independently → Operations contracts locked
7. Polish → Cross-document consistency check

### Parallel Team Strategy

With multiple reviewers:
1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Reviewer A: US1 (architecture contracts) + US3 (events)
   - Reviewer B: US2 (database schema) + US4 (RBAC) + US5 (CI/CD)
3. Stories complete and integrate independently — no file conflicts
   (each contract is a unique file)
