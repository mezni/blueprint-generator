# Implementation Plan: Backend Integration

**Branch**: `002-backend-integration` | **Date**: 2026-05-27 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/002-backend-integration/spec.md`

## Summary

Add a Rust Actix-web backend API service (`api-service`) that serves EV charging station data to the mobile driver app. The frontend fetches stations from `GET /api/v1/stations/nearby`, displays colored map markers (green=Available, red=Occupied), and shows a detail card on tap. CI validates both backend (Rust) and frontend (Expo) builds.

## Technical Context

**Language/Version**: Rust 2021 edition (Actix-web 4.4) for backend; JavaScript/JSX (React Native 0.74 via Expo SDK 51) for frontend

**Primary Dependencies**: actix-web 4.4, serde 1.0, chrono 0.4, parking_lot 0.12 (backend); react-native-maps 1.14, axios ^1.6 (frontend)

**Storage**: In-memory mock data (Vec<Station>) behind RwLock — no persistent storage in v1

**Testing**: `cargo test` for backend unit tests; manual visual verification on device for frontend; CI build verification via `npx expo export --platform web`

**Target Platform**: Linux server (backend); iOS/Android via Expo Go (frontend)

**Project Type**: web-service + mobile-app

**Performance Goals**: API responds in under 500ms for small mock dataset; map viewport renders initial stations within 5 seconds of app launch

**Constraints**: Backend runs on local network (configurable via EXPO_PUBLIC_API_URL); offline-capable error screen when backend unreachable; no persistent database in v1

**Scale/Scope**: 1 backend service (api-service), 1 mobile app (mobile-driver), 2 mock stations initially

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Validation Before Optimization | ✅ PASS | Mock data with no caching/brokers — simplicity preserved |
| II. Technical Stack Governance | ✅ PASS | Rust/Actix-web backend, React Native/Expo frontend per locked stack |
| III. API & Service Architecture | ✅ PASS | `api-service` gateway with `/api/v1` prefix — aligns with architectural standard |
| IV. Data Architecture Standards | ✅ PASS | `stn-xxx`/`chg-xxx` ID patterns used; Tunis center coordinates respected |
| V. Development & Environment Discipline | ✅ PASS | Offline-first mobile testing; no Docker dependency for mock data phase |
| Additional Constraints | ✅ PASS | No Redis, RabbitMQ, or persistent database in this phase |
| Development Workflow & Quality Gates | ✅ PASS | API versioned under `/api/v1`; CI validates both backend and frontend |

**Gate Result**: ALL PASS — No violations requiring complexity justification.

## Project Structure

### Documentation (this feature)

```text
specs/002-backend-integration/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── checklists/
    └── requirements.md  # Spec quality checklist
```

### Source Code (repository root)

```text
borne-map/
├── .github/
│   └── workflows/
│       └── ci.yml               # Rust + Expo CI pipeline
├── apps/
│   └── mobile-driver/           # React Native / Expo Go
│       ├── package.json
│       ├── App.js
│       └── src/
│           ├── components/
│           │   └── StationCard.js
│           ├── screens/
│           │   └── MapScreen.js
│           └── services/
│               └── api.js
└── backend/                     # Rust workspace
    ├── Cargo.toml
    ├── api-service/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       └── domains/
    │           └── locate/
    │               ├── mod.rs
    │               ├── model.rs
    │               └── routes.rs
    └── core/
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

**Structure Decision**: Backend uses Rust workspace with `api-service` as the main gateway and `core` as a shared library. Frontend uses Flat-Nested pattern under `apps/mobile-driver/`. CI pipeline in `.github/workflows/`. This follows the Integrated Domain Architecture Layout from the spec.

## Complexity Tracking

*No constitutional violations — complexity justification not required.*
