# MVP 1 — Geo Core Validation Report

**Date**: _to be filled_
**Version**: _to be filled_
**Cohort**: _≥5 drivers + ≥2 admins_

## Cohort Composition

| # | Role | Device | OS Version | Notes |
|---|------|--------|------------|-------|
| 1 | Driver | | | |
| 2 | Driver | | | |
| 3 | Driver | | | |
| 4 | Driver | | | |
| 5 | Driver | | | |
| 6 | Admin | | | |
| 7 | Admin | | | |

_Performance reference devices (at least one of each): Samsung Galaxy A33 5G, iPhone 11_
_Compatibility floor devices (at least one per platform): Android 10, iOS 15_

## Task Success Rate

| Task | Scenario | Successes | Failures | Success Rate |
|------|----------|-----------|----------|-------------|
| T1 | Find nearest charger to given address (≤30s) | | | |
| T2 | Open detail view of chosen pin | | | |
| T3 | Navigate from detail view to OS map | | | |
| T4 | Admin create station (≤2 min) | | | |
| T5 | Admin edit station (toggle maintenance) | | | |
| T6 | Admin soft-delete station | | | |

## Success Criteria

| ID | Criterion | Target | Measured | Pass? |
|----|-----------|--------|----------|-------|
| SC-001 | P95 server latency (viewport query) | ≤200 ms | | |
| SC-002 | Map FPS on reference devices | ≥60 FPS | | |
| SC-003 | Cold-start to first marker (web/mobile) | ≤2.5s / ≤3.5s | | |
| SC-004 | Driver finds nearest charger ≤30s | ≥90% | | |
| SC-005 | Driver opens detail view | ≥95% | | |
| SC-006 | Admin creates station ≤2 min | ≥90% | | |
| SC-007 | Zero hand-written HTTP types | 0 | | |
| SC-008 | GiST index on every spatial column | all | | |
| SC-009 | No unwrap/expect/panic in non-test Rust | 0 | | |

## SUS Score

_Average System Usability Scale across all participants: __ / 100_

## Qualitative Feedback

_Driver feedback:_

_Admin feedback:_

## Usability Blockers (P0)

_None identified / List below:_

## Decision

- [ ] **PROCEED** to MVP 2
- [ ] **ADJUST** — changes required before proceeding (list below)
- [ ] **KILL** — fundamental flaw identified

_Decision made by: _  _Date: _
