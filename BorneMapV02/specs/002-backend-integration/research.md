# Research: Backend Integration

## Overview

Research conducted to resolve technical unknowns and validate approach for the backend integration feature.

## Technology Decisions

### Backend Framework
- **Decision**: Actix-web 4.4 (Rust)
- **Rationale**: Per constitutional stack governance — Rust + Actix-web is the locked backend framework. Actix-web 4.4 provides async HTTP, middleware support (Logger), and mature ecosystem.
- **Alternatives considered**: Axum (Tokio ecosystem but not in locked stack), Rocket (sync-first, less idiomatic for async)

### API Versioning
- **Decision**: `/api/v1/` URL prefix
- **Rationale**: Matches constitutional requirement for versioned public API endpoints. `/api/v1/stations/nearby` allows future v2 without breaking existing clients.
- **Alternatives considered**: Header-based versioning (Accept header), no versioning (too risky)

### Data Format
- **Decision**: JSON over HTTP REST
- **Rationale**: Standard for mobile-backend communication; natively supported by serde (Rust) and axios (JS); human-readable for debugging.
- **Alternatives considered**: Protocol Buffers (overkill for v1 mock data), GraphQL (excessive complexity for 2 endpoints)

### Identifier Format
- **Decision**: `XXX-nanouuid` pattern (`^[a-z]{3}-[a-f0-9]{8}$`)
- **Rationale**: Per constitutional data architecture standard (IV). Three-letter prefix encodes entity type (stn, chg, prv); 8-hex-char suffix provides collision resistance without full UUID verbosity.
- **Alternatives considered**: Auto-increment integers (barred), standard UUIDv4 (barred by constitution), ULID (not in standard)

### State Management
- **Decision**: In-memory RwLock<Vec<Station>> within AppState
- **Rationale**: Simplest possible data layer for mock data; parking_lot::RwLock provides fast concurrent reads. Sufficient for v1 until a real database is introduced.
- **Alternatives considered**: PostgreSQL (unnecessary complexity for mock phase), Arc<Mutex> (RwLock better for read-heavy workloads)

### Frontend HTTP Client
- **Decision**: Axios for React Native
- **Rationale**: Mature, promise-based HTTP client with interceptors; environment-aware URL via `EXPO_PUBLIC_API_URL`; consistent error handling.
- **Alternatives considered**: fetch (native but less ergonomic for error handling), react-query (overkill for single endpoint)

### CI Pipeline
- **Decision**: GitHub Actions with two parallel jobs (backend Rust, frontend Expo)
- **Rationale**: Native GitHub Actions support for both Rust (dtolnay/rust-toolchain) and Node.js (actions/setup-node). Parallel jobs minimize total CI time.
- **Alternatives considered**: Single job serial (slower), self-hosted runner (unnecessary for v1)

## Mock Data Design

Two stations serving as representative test cases:
1. Available station with 120kW CCS2 charger (LES BERGES DU LAC 2 HUB)
2. Occupied station with 50kW CCS2 charger (TUNIS MARINE PLAZA)

This covers both status states (green/red markers) and shows a power output range.

## API Contract Summary

```
GET /api/v1/stations/nearby → 200 OK
Response: Station[] (see data-model.md for schema)
Error: Service unavailable (connection refused, timeout)
```

No authentication for v1 — backend is local-network only.
