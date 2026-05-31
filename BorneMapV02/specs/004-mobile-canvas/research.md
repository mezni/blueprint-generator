# Research: Mobile Canvas

**Phase**: 0 | **Date**: 2026-05-28

## Overview

No unresolved technology choices or clarifications remained after the spec clarification phase. All technical decisions are determined by the project constitution and existing codebase conventions.

## Decisions

### Decision 1: Enum Renaming Strategy

- **Decision**: Create a new migration that drops and recreates the schema (destructive) since all current data is demo/development only.
- **Rationale**: Simplest approach with no data loss risk for test data. Avoids complex ALTER TYPE migration logic.
- **Alternatives considered**: Inline ALTER TYPE migration (more complex, needed for production data).

### Decision 2: Status Lifecycle Model

- **Decision**: Extended status set with four values: Available, Occupied, Offline, Maintenance. Transitions: Available ↔ Occupied, Available → Offline → Available, Available → Maintenance → Available.
- **Rationale**: Covers real-world charging station states beyond simple binary availability.
- **Alternatives considered**: Binary (Available/Occupied only) — insufficient for maintenance/offline scenarios.

### Decision 3: Frontend Error Handling

- **Decision**: Loading spinner during fetch, persistent error banner with retry button on failure.
- **Rationale**: Simplest UX pattern for a mobile app; clear user feedback without over-engineering.
- **Alternatives considered**: Skeleton loaders (higher implementation effort), silent background retry (confusing when stale data shown).

### Decision 4: Identifier Pattern Enforcement

- **Decision**: CHECK constraints on all three entity tables enforcing `^prt-[a-f0-9]{8}$`, `^stn-[a-f0-9]{8}$`, `^chg-[a-f0-9]{8}$` regex patterns.
- **Rationale**: Database-level enforcement is the most reliable layer; prevents invalid data regardless of application behavior.
- **Alternatives considered**: Application-level validation only (bypassable), UUID v4 (constitution prohibits).

## Technology Stack (Constitution-Locked)

All technology choices are LOCKED by the project constitution (v1.0.0) and are not open for research:

| Layer | Technology | Constitution Reference |
|-------|-----------|----------------------|
| Backend | Rust (Actix-web 4.4) | Principle II |
| Database | PostgreSQL 15 + PostGIS 3.3 | Principle II |
| Mobile | React Native (Expo SDK 51) | Principle II |
| Admin Portal | React (Vite/Next.js) | Principle II |
| Runtime | Node.js v24.16.0 | Principle II |
| DB Driver | sqlx 0.8 with postgres feature | Existing project choice |
| Serialization | serde 1.0 + serde_json | Existing project choice |
| Spatial | PostGIS GEOGRAPHY(Point, 4326) | Principle IV |
