# Feature Specification: MVP 4 — Performance Layer

**Feature Branch**: `004-mvp4-performance`
**Created**: 2026-05-23
**Status**: Draft
**Input**: User description: "Cut spatial P95 under realistic load by adding a Redis cache on quantized viewport keys; tune indexes and clustering."

> Predecessors: MVPs 1–3 merged and validated.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Cached viewport (Priority: P1)

A driver pans across a city; repeated viewports hit Redis instead of PostGIS, keeping the map snappy.

**Independent Test**: With Redis warmed, repeated identical viewport queries return in ≤ 50 ms P95 (server-side).

**Acceptance Scenarios**:

1. **Given** Redis is empty, **When** the first viewport query runs, **Then** it executes PostGIS and populates `stations:bbox:{w}:{s}:{e}:{n}` with TTL 60 s.
2. **Given** the same quantized bbox is queried again, **When** the cache is hit, **Then** the response is served from Redis with the cache header `X-Cache: HIT`.
3. **Given** an admin updates a station inside a known bbox, **When** the update commits, **Then** the relevant cache key is invalidated (or expires within TTL — chosen strategy must be documented in `plan.md` of this MVP).

---

### User Story 2 — Load-tested at scale (Priority: P1)

The system handles 10 000 seeded stations and 200 concurrent viewport queries/s for 5 minutes without latency regression.

**Independent Test**: A k6 (or equivalent) script reports P95 ≤ 80 ms warm and ≤ 200 ms cold under the documented load profile.

---

### User Story 3 — Cluster tuning (Priority: P2)

Dense urban viewports (Tunis center, zoom 13–16) display clean clusters without visible flicker on pan/zoom.

**Independent Test**: Qualitative pass during validation sessions (recorded video review) with at least 3 cohort members.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST integrate Redis 7 as a read-through cache on `GET /api/v1/stations`.
- **FR-002**: Cache key format: `stations:bbox:{w}:{s}:{e}:{n}` using 4-decimal quantized values.
- **FR-003**: Cache TTL: configurable via env `STATION_BBOX_TTL_SECONDS` (default 60).
- **FR-004**: Cache hit/miss MUST be exposed as Prometheus counters `bornemap_station_cache_hits_total` and `bornemap_station_cache_misses_total`.
- **FR-005**: `POST/PATCH/DELETE /api/v1/admin/stations` MUST publish a cache-invalidation message (or rely on TTL — choice documented in this MVP's `plan.md`).
- **FR-006**: A k6 load profile MUST live under `infrastructure/loadtest/` and be runnable from CI nightly.
- **FR-007**: GiST index review: `EXPLAIN ANALYZE` outputs MUST be committed under `specs/004-…/research.md` for the canonical viewport query at 1k, 10k, and 50k stations.
- **FR-008**: Cluster threshold (R7) MUST remain configurable: `CLUSTER_DENSITY=15`, `CLUSTER_RADIUS_PX=40`.

### Key Entities

- **CacheEntry**: Redis string containing the serialized marker list for a quantized bbox.
- **LoadProfile**: A k6 script defining users, RPS, duration, viewport-walk pattern over Tunisia.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Steady-state cache hit ratio ≥ **70%** during a 1-hour realistic usage simulation.
- **SC-002**: P95 viewport query (warm cache) ≤ **80 ms** server-side.
- **SC-003**: P95 viewport query (cold cache, 10 k stations) ≤ **200 ms**.
- **SC-004**: Cluster behavior at zoom 13–16 in dense urban viewports passes qualitative review by ≥ 3 cohort members.
- **SC-005**: Nightly load-test report archived; no regression vs previous run > 10%.

### Proceed Criteria → MVP 5

All SC-001..SC-005 met; ≥ 5 driver interviews report no staleness complaints; load-test report attached.

## Assumptions

- Redis is single-node for MVP 4; clustering deferred until justified.
- Cache invalidation strategy chosen and documented in this MVP's `plan.md` (preferred: short TTL + targeted invalidation on admin writes).
- Load profile mimics Tunis-center hotspots and inter-city pans.

## Constitutional Gates

- [ ] No degraded-ops violation (fallback to static map snapshot if avg spatial latency > 1500 ms in 15-min window).
- [ ] No new public ingress added.
- [ ] Bounds Cache Registry Rule (R6) consistent between client and server keys.
