# Feature Specification: MVP 2 — User Interaction Layer

**Feature Branch**: `002-mvp2-user-interaction`
**Created**: 2026-05-23
**Status**: Draft
**Input**: User description: "Turn the read-only directory into an interactive product. Drivers create accounts (mock JWT), favorite stations, leave reviews. Admins moderate. Token claim shape remains frozen."

> Predecessors: MVP 1 must be merged and validated.
> Read first: [`docs/plan.md`](../../docs/plan.md) §MVP 2 + [`docs/constitution.md`](../../docs/constitution.md).

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Driver signs up and reviews a station (Priority: P1)

A new driver signs up via email/password (mock JWT issuance) and submits a 1–5 star review with a short comment for a station they visited.

**Why this priority**: The review loop is the platform's core social proof; without it, MVP 2 has no product.

**Independent Test**: A fresh user can sign up, see their session persist, open a station, leave a review, and see it on the station's detail page in ≤ 90 s.

**Acceptance Scenarios**:

1. **Given** an unauthenticated visitor, **When** they submit `{email, password, display_name}`, **Then** they receive a JWT and their `profile_domain.driver_profiles` row exists.
2. **Given** an authenticated driver on a station detail page, **When** they submit a review with rating 4 and a comment, **Then** the review appears in `GET /api/v1/stations/{id}/reviews`.
3. **Given** the driver opens the same page again, **When** the list renders, **Then** their own review is visible with a "Delete" affordance.
4. **Given** a driver attempts to submit a 0-star or 6-star review, **When** they submit, **Then** the server rejects with HTTP 422 and an RFC-7807 problem detail.

---

### User Story 2 — Driver favorites stations (Priority: P1)

A driver toggles a heart icon on a station to add it to favorites, then views the list of their favorites on a dedicated page.

**Why this priority**: Favorites drive return visits and feed future personalization without requiring social features.

**Independent Test**: A driver favorites 3 stations across different sessions; the favorites list shows all 3 in any order.

**Acceptance Scenarios**:

1. **Given** an authenticated driver, **When** they POST `/api/v1/favorites { station_id }`, **Then** the favorite persists and the heart icon reflects the new state on next render.
2. **Given** an existing favorite, **When** the driver DELETEs `/api/v1/favorites/{station_id}`, **Then** it is removed.
3. **Given** the driver opens "My Favorites", **When** the page loads, **Then** all current favorites render with name, address, and a link to the station's detail page.

---

### User Story 3 — Admin moderates reviews (Priority: P2)

An admin opens the moderation queue, filters by status, and hides or flags a review.

**Why this priority**: Required to keep the catalog clean; not the driver's core product loop.

**Independent Test**: An admin can move a `published` review to `hidden` and confirm it disappears from public `GET /api/v1/stations/{id}/reviews`.

**Acceptance Scenarios**:

1. **Given** an admin token, **When** they GET `/api/v1/admin/reviews?status=published`, **Then** they receive paginated published reviews.
2. **Given** a flagged review, **When** the admin PATCHes it to `hidden`, **Then** the public endpoint no longer returns it.

---

### Edge Cases

- Duplicate signup with the same email: server returns 409 with RFC-7807 detail.
- Driver attempts to review the same station twice: server allows one review per `(station_id, author_id)` (DB unique constraint) and returns 409 on second attempt.
- Driver deletes their account (out of scope MVP 2): tracked as `[DEFERRED MVP 3+]`.
- Race on first-login profile init: `ON CONFLICT (user_id) DO UPDATE` (Principle X).
- Token from MVP 1 admin login still works (claim shape is frozen).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST expose `POST /api/v1/auth/mock-signup` and `POST /api/v1/auth/mock-login`, issuing JWTs with the **canonical claim shape unchanged from MVP 1**.
- **FR-002**: System MUST hash passwords using `argon2id`.
- **FR-003**: System MUST expose `GET /api/v1/profile/me`, `PATCH /api/v1/profile/me`. Profile initialization MUST use `INSERT … ON CONFLICT (user_id) DO UPDATE`.
- **FR-004**: System MUST expose `POST /api/v1/stations/{id}/reviews`, `GET /api/v1/stations/{id}/reviews`, `DELETE /api/v1/reviews/{id}` (author-only).
- **FR-005**: System MUST enforce one review per `(station_id, author_id)` via a UNIQUE constraint.
- **FR-006**: System MUST expose `POST /api/v1/favorites { station_id }`, `DELETE /api/v1/favorites/{station_id}`, `GET /api/v1/favorites`.
- **FR-007**: System MUST expose `GET /api/v1/admin/reviews?status=…`, `PATCH /api/v1/admin/reviews/{id} { status }`.
- **FR-008**: Pin colors MUST remain backend-computed (`is_active`, `under_maintenance`) — no client override.
- **FR-009**: All endpoints that return collections MUST support cursor-based pagination (`limit`, `cursor`).
- **FR-010**: All new endpoints MUST have contract + integration tests (PostGIS Testcontainers).

### Key Entities

- **User**: Email-identified account with hashed password and role (`driver` | `admin`).
- **DriverProfile**: Display name, locale, avatar URL (string only; upload deferred to MVP 3).
- **Review**: 1–5 rating, body, status (`published` | `hidden` | `flagged`), author, station.
- **Favorite**: `(user_id, station_id)` join; idempotent toggle.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Driver signup completion ≤ **90 s** for ≥ 90% of cohort.
- **SC-002**: Review submission success rate ≥ **95%** across cohort.
- **SC-003**: Favorite toggle client-perceived latency ≤ **200 ms**.
- **SC-004**: A single admin can triage ≥ **50 reviews/hour** in the moderation queue.
- **SC-005**: Token claim shape diff vs MVP 1 = **0** (verified by snapshot test).
- **SC-006**: No frontend filter-after-fetch of server collections (grep + code review).

### Proceed Criteria → MVP 3

All SC-001..SC-006 met; ≥ 10 driver interviews + ≥ 2 admin interviews; no Medium+ security finding.

## Assumptions

- No social federation yet; signup uses internal credentials only.
- Avatars referenced via URL string (e.g., gravatar-style or user-pasted) — upload pipeline lands in MVP 3.
- Email verification is out of scope for MVP 2 (deferred to MVP 5 via Keycloak).
- Reviews are public (any visitor reads them); only authors can edit/delete their own.

## Constitutional Gates

- [ ] Idempotent Profile Rule (Principle X) — exact `ON CONFLICT DO UPDATE` SQL.
- [ ] Frozen claim shape (Principle II + §6).
- [ ] Backend-only filtering & authorization.
- [ ] OpenAPI updated; generated client refreshed.
- [ ] Real-user validation report attached.
