# Feature Specification: MVP 6 — Scale & Extraction (Conditional)

**Feature Branch**: `006-mvp6-scale`
**Created**: 2026-05-23
**Status**: **Draft — GATED**
**Input**: User description: "Decompose the modular monolith into per-domain Rust services behind a BFF gateway, with tonic gRPC interconnect and OpenTelemetry distributed tracing. Only proceed if justified by production load benchmarks."

> ⚠️ **GATE**: This MVP does NOT start until `specs/006-mvp6-scale/justification.md` is written and approved. See "Gate" below.

## Gate (Mandatory Pre-Read)

Open and complete `specs/006-mvp6-scale/justification.md` **before** any other work in this spec. The file must:

1. Provide production metrics demonstrating **one or more** of:
   - Sustained CPU or memory saturation unresolvable by vertical scaling (attach Grafana screenshots / Prometheus queries).
   - Per-domain traffic skew ≥ 10× across the monolith (attach per-handler request counters).
   - Regulatory / tenancy / contractual isolation requirement (attach the source document).
2. Be reviewed and signed off by the platform owner of record.
3. Be referenced from `Complexity Tracking` in this MVP's `plan.md`.

**Without an approved justification, this MVP MUST NOT start.** The platform stays on the modular monolith.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Internal: zero-downtime extraction of `station-service` (Priority: P1)

The platform operator extracts the station domain into its own service; users experience no downtime; functional behavior is unchanged.

**Independent Test**: A canary deployment routes 5% → 50% → 100% of traffic through the BFF + extracted service, with error rates and P95 latency within the regression budget (≤ +50 ms vs MVP 5).

**Acceptance Scenarios**:

1. **Given** canary at 5%, **When** synthetic monitoring runs for 30 min, **Then** error rate and latency match SLO targets.
2. **Given** full cutover, **When** users use the platform for 24 h, **Then** no user-facing regression is reported.

---

### User Story 2 — Cross-service distributed tracing (Priority: P1)

A request hitting the BFF produces a single distributed trace spanning the gateway and downstream services.

**Independent Test**: A scripted request to `GET /api/v1/stations` produces a Jaeger/Tempo trace with spans from `bff`, `station-service`, and `postgres`.

---

### User Story 3 — Rollback drill (Priority: P1)

The team can re-merge a service back into the monolith if a regression appears.

**Independent Test**: Execute the documented rollback procedure in staging; verify behavior is preserved and traffic flows back through the monolith path.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST split `bornemap-backend` into separate binaries: `station-service`, `identity-service`, `profile-service`, `review-service`, `event-service`, plus `api-gateway` (BFF).
- **FR-002**: Inter-service RPC MUST use `tonic` (gRPC) over the private app subnet.
- **FR-003**: Each service MUST own a distinct PostgreSQL database (or distinct DB role with exclusive schema ownership). No shared schemas.
- **FR-004**: The BFF MUST be stateless and aggregate cross-domain responses where needed.
- **FR-005**: All cross-service calls MUST propagate `traceparent` headers and feed OpenTelemetry collectors.
- **FR-006**: The OpenAPI contract MUST remain stable at the BFF edge — clients (web/mobile) MUST NOT need code changes beyond regenerated bindings.
- **FR-007**: A documented rollback procedure (`specs/006-…/rollback.md`) MUST exist and have been dry-run.
- **FR-008**: Canary deployment configuration (traffic-shift policy) MUST be committed under `infrastructure/`.

### Key Entities

- **Service**: An independently deployable Rust binary with its own database role.
- **TraceContext**: W3C `traceparent` propagated end-to-end.
- **Justification**: The document gating this MVP (see Gate section).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: ≥ **99%** of cross-service requests carry a single distributed trace ID end-to-end.
- **SC-002**: P95 latency at the BFF ≤ MVP 5 P95 **+ 50 ms** (regression budget).
- **SC-003**: **Zero** shared schemas across services (verified by DB role privilege audit).
- **SC-004**: Documented rollback procedure dry-run completed successfully in staging.
- **SC-005**: Justification document approved before merge of any extraction PR.

### Proceed Criteria → (none)

This is the terminal MVP in the current roadmap. Future MVPs require a new plan revision.

## Assumptions

- Internal network is sufficiently low-latency to absorb the gRPC hop overhead within the regression budget.
- Existing OpenAPI surface is stable enough that a BFF aggregator can preserve it without breaking changes.
- Operational tooling (Jaeger or Tempo + Grafana) is available for distributed-tracing review.

## Constitutional Gates

- [ ] `justification.md` written and approved.
- [ ] Per-service database ownership enforced.
- [ ] BFF preserves API contract — clients change only via regenerated bindings.
- [ ] OpenTelemetry collectors deployed and verified.
- [ ] Rollback procedure exists and has been dry-run.
