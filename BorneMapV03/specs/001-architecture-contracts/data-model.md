# Data Model: Architecture Contracts

## Overview

This feature produces 8 contract documents that collectively define the
BorneMap platform's system constitution. These documents are the "data model"
of this feature — they are structured, versioned artifacts with defined
relationships.

## Contract Documents

| Document | Purpose | Dependencies |
|----------|---------|-------------|
| `architecture-contract.md` | Service boundaries, communication model, invariants | None |
| `service-matrix.md` | Per-service responsibilities, owned tables, tech stack | architecture-contract.md |
| `event-spec-v1.md` | Clickstream event envelope, types, validation rules | None |
| `rbac-model.md` | Three roles, enforcement layers, partner isolation | None |
| `id-strategy.md` | NanoID prefixes, format rules | None |
| `communication-rules.md` | REST + RabbitMQ rules, sync vs async | architecture-contract.md |
| `ci-cd-contract.md` | Pipeline stages, GHCR rules, build requirements | None |
| `database-schema-contract.md` | Four schemas, tables, constraints, partitioning | None |

## Entity-Relationship (Cross-Document)

```text
architecture-contract.md
  ├── referenced by: service-matrix.md (owns logic)
  └── referenced by: communication-rules.md (implements rules)

database-schema-contract.md
  ├── referenced by: service-matrix.md (data ownership)
  └── referenced by: communication-rules.md (DB access rules)

event-spec-v1.md
  └── referenced by: ci-cd-contract.md (contract validation stage)

rbac-model.md
  ├── referenced by: architecture-contract.md (auth model)
  └── referenced by: ci-cd-contract.md (auth test requirements)

id-strategy.md
  └── referenced by: database-schema-contract.md (PK ID columns)

ci-cd-contract.md
  └── depends on: all other contracts for pipeline design
```

## Document Structure (Template)

Each contract document follows this structure:

```text
# [Document Title]

## Purpose
[One-paragraph statement of what this document defines]

## Version
[Version number, related to constitution version]

## Contracts
[Bullet list or table of specific contract rules]

## Enforcement
[How compliance with this contract is verified]

## Exceptions
[Any allowed deviations and their approval process]
```

## Relationship to Constitution

The constitution (`docs/constitution.md`, `.specify/memory/constitution.md`) is
the parent document. The contracts in this feature are a more detailed,
implementation-focused breakdown of the constitution's sections.

```text
Constitution (high-level principles)
  └── EPIC 0 Contracts (detailed specifications for each domain)
        └── Service implementations (conform to contracts)
```
