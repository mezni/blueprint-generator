# Implementation Plan: Mobile Driver App Scaffold

**Branch**: `001-mobile-driver-scaffold` | **Date**: 2026-05-27 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/001-mobile-driver-scaffold/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.

## Summary

Scaffold the initial BorneMap mobile driver application with a standardized project
directory layout, GitHub Actions CI pipeline, and an offline React Native map view
centered on Tunis coordinates with a diagnostic debug overlay. No backend or database
dependencies — purely a frontend diagnostic checkpoint.

## Technical Context

**Language/Version**: JavaScript/JSX — React Native 0.74.1 via Expo SDK 51

**Primary Dependencies**: react-native-maps 1.14.0 for map rendering; expo ~51.0.0
for the managed workflow runtime

**Storage**: N/A (no data persistence in this diagnostic scaffold phase)

**Testing**: CI build verification via `npx expo export --platform web`;
manual visual verification on physical device via Expo Go

**Target Platform**: iOS (Expo Go), Android (Expo Go); CI targets web export only

**Project Type**: mobile-app (React Native / Expo managed workflow)

**Performance Goals**: Map viewport renders initial region within 5 seconds of
app launch; CI build completes in under 3 minutes on standard GitHub Actions runner

**Constraints**: Offline-capable — zero network requests from app code; no backend
or API dependency; must run entirely in Expo Go managed workflow sandbox

**Scale/Scope**: Single-screen diagnostic scaffold; 1 developer target; 1 app (mobile-driver)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Validation Before Optimization | ✅ PASS | Diagnostic scaffold validates render pipeline first; no caching/brokers |
| II. Technical Stack Governance | ✅ PASS | Uses locked stack: Expo SDK 51, React Native 0.74.1, Node.js 24 |
| III. API & Service Architecture | ✅ PASS | Not applicable — no backend service in this phase |
| IV. Data Architecture Standards | ✅ PASS | Default view anchors on Tunis (36.8065, 10.1815) as required |
| V. Development & Environment Discipline | ✅ PASS | Offline maps with hardcoded coords; tunnel for device testing |
| Additional Constraints | ✅ PASS | No Redis/RabbitMQ; no dependencies outside locked stack |
| Development Workflow & Quality Gates | ✅ PASS | Offline-first UX diagnostics per constitutional requirement |

**Gate Result**: ALL PASS — No violations requiring complexity justification.

## Project Structure

### Documentation (this feature)

```text
specs/001-mobile-driver-scaffold/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── spec.md              # Feature specification
```

### Source Code (repository root)

```text
borne-map/
├── .github/
│   └── workflows/
│       └── ci.yml               # GitHub Actions CI pipeline
└── apps/
    └── mobile-driver/           # React Native / Expo Go
        ├── package.json         # Dependencies & scripts
        ├── App.js               # Application entrypoint
        └── src/
            └── screens/
                └── MapScreen.js # Offline map viewport with debug overlay
```

**Structure Decision**: Single mobile app project under `apps/mobile-driver/` with
Flat-Nested pattern — App.js at root, screen components in `src/screens/`.
CI pipeline in `.github/workflows/`. No backend or library packages in this phase.

## Complexity Tracking

*No constitutional violations — complexity justification not required.*
