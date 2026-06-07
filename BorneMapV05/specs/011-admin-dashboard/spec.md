# Feature Specification: Admin Dashboard

**Feature Branch**: `011-admin-dashboard`

**Created**: 2026-06-04

**Status**: Draft

## Clarifications

### Session 2026-06-04

- Q: Should the Reports page be implemented this sprint or deferred? → A: Defer Reports page to Sprint 14/15. US6 and FR-014 are out of scope for this sprint.
- Q: What layout pattern should the admin dashboard use? → A: Same 260px sidebar + main content layout as partner-dashboard (consistent UX).
- Q: Should clickstream events be emitted this sprint or deferred? → A: Wire clickstream emission now using the same pattern as partner-dashboard.
- Q: How should chargers appear in the admin station view? → A: Charger count column in station list + expandable charger details in station detail view.
- Q: Should the admin be able to edit station coordinates (lat/lng)? → A: Include lat/lng with confirmation dialog on change (GIS resync handles the rest).

**Input**: User description: "Sprint 11 Admin Dashboard — global control interface for administrators to manage partners, stations, reviews, users, and system overview"

## User Scenarios & Testing

### User Story 1 — System Overview Dashboard (Priority: P1)

An admin logs in and sees a summary dashboard with key platform metrics: total partners, total stations, active stations, pending reviews, and recent activity. This is the landing page that gives the admin immediate visibility into platform health.

**Why this priority**: Without an overview, the admin has no starting point to understand platform state or identify issues requiring action.

**Independent Test**: Can be fully tested by authenticating as admin and navigating to the dashboard root; all metric cards render with correct counts from the platform.

**Acceptance Scenarios**:

1. **Given** the admin is authenticated, **When** they land on the dashboard, **Then** they see a header and a grid of metric cards (partners, stations, active stations, pending reviews).
2. **Given** the admin views a metric card, **When** the data is loading, **Then** a skeleton placeholder is displayed.
3. **Given** the dashboard is loaded, **When** the API returns an error, **Then** an error state with a retry option is shown.
4. **Given** the dashboard is displayed, **When** the admin clicks a metric card (e.g., Partners), **Then** they navigate to the corresponding management page.

---

### User Story 2 — Partner Management (Priority: P1)

An admin views, creates, updates, and deletes partners. The partner list shows all registered partners with their status (active/suspended). The admin can create new partners, edit partner details, or delete a partner (blocked if the partner has active stations).

**Why this priority**: Partner management is the foundational data entity — all stations and chargers belong to a partner. Without this, the admin cannot onboard or manage the platform's core business entities.

**Independent Test**: Can be fully tested by listing partners, creating a new partner, editing its name/status, and verifying that deletion is blocked when active stations exist.

**Acceptance Scenarios**:

1. **Given** the admin is on the Partners page, **When** they view the list, **Then** all partners are shown in a paginated table with name, type, status, and creation date.
2. **Given** the admin clicks "Add Partner", **When** they fill in the form and submit, **Then** the new partner appears in the list.
3. **Given** the admin edits a partner, **When** they change the status to "suspended", **Then** the partner's stations should become non-operational.
4. **Given** the admin attempts to delete a partner, **When** that partner has active stations, **Then** the system rejects the deletion with a clear error message.
5. **Given** the admin deletes a partner, **When** that partner has no active stations, **Then** the partner is soft-deleted (removed from the active list).

---

### User Story 3 — Station Management (Priority: P2)

An admin views all stations across all partners with the ability to edit station details and soft-delete stations. The station list shows every station on the platform with its partner, status, location, and charger count.

**Why this priority**: Administrators need global visibility into all stations for platform oversight and moderation, but this is secondary to core partner management.

**Independent Test**: Can be fully tested by viewing all stations in a paginated list, editing a station's status, and soft-deleting a station.

**Acceptance Scenarios**:

1. **Given** the admin is on the Stations page, **When** they view the list, **Then** all stations across all partners are shown with partner name, status, city, live/public flags, and charger count.
2. **Given** the admin clicks a station, **When** they edit its status to "maintenance", **Then** the change is reflected immediately.
3. **Given** the admin soft-deletes a station, **When** they view the list with default filters, **Then** the deleted station is hidden.
4. **Given** the admin toggles "show deleted", **When** they view the list, **Then** soft-deleted stations appear with a visual indicator.
5. **Given** the admin clicks a station row, **When** the detail view expands, **Then** they see a list of chargers with type, power_kw, and status.

---

### User Story 4 — Review Moderation (Priority: P2)

An admin views all station reviews and can moderate their status through the review lifecycle (submitted → published → flagged → hidden).

**Why this priority**: Review moderation ensures content quality and handles abuse reports. Moderating review status is a core admin responsibility.

**Independent Test**: Can be fully tested by viewing reviews, changing a review's status from "published" to "hidden", and verifying the change is persisted.

**Acceptance Scenarios**:

1. **Given** the admin is on the Reviews page, **When** they view the list, **Then** all reviews are shown with station, user, rating, comment preview, and current status.
2. **Given** the admin selects a review, **When** they change its status to "hidden", **Then** the review disappears from public view.
3. **Given** the admin attempts an invalid status transition, **When** the API rejects it, **Then** the UI shows an error message.

---

### User Story 5 — User Management (Priority: P3)

An admin views all registered users in a paginated table with their email, role, status, and last login time.

**Why this priority**: User visibility is useful for support and auditing but is a read-only view with no platform-critical impact.

**Independent Test**: Can be fully tested by listing users and verifying the data matches Keycloak/user_account records.

**Acceptance Scenarios**:

1. **Given** the admin is on the Users page, **When** they view the list, **Then** all users are shown with email, status, and last login time.
2. **Given** the user list is displayed, **When** the admin searches by email, **Then** results are filtered accordingly.

---

### User Story 6 — Reports Overview (DEFERRED to Sprint 14/15)

**Deferred**: Reports require analytics data from the analytics pipeline (Sprint 14+). Not implemented in this sprint.

### Edge Cases

- What happens when the admin's token expires mid-session? The app should redirect to login and resume on re-auth.
- How does the system handle empty states (no partners, no stations, no reviews)? Each list page should show a friendly empty state with guidance.
- How are concurrent edits handled? The API uses optimistic locking via `If-Match` header — stale edits should show a conflict error.
- How should the UI behave when an API endpoint returns a 403? Show a "you don't have permission" message rather than a generic error.
- What happens when a partner deletion is blocked by active stations? The UI should show which stations are blocking the deletion.
- How are paginated lists handled across page refreshes? Page state should reset to page 1 on navigation, not persist stale pages.

## Requirements

### Functional Requirements

- **FR-001**: Admin MUST be able to authenticate via Keycloak and access all admin routes.
- **FR-002**: System MUST display a dashboard overview with counts of partners, stations, active stations, and pending reviews.
- **FR-003**: Admin MUST be able to list all partners with pagination, search, and status filter.
- **FR-004**: Admin MUST be able to create new partners with name, type, and status fields.
- **FR-005**: Admin MUST be able to edit partner details including name, type, and status.
- **FR-006**: Admin MUST be able to soft-delete a partner (blocked if active stations exist).
- **FR-007**: Admin MUST be able to list all stations across all partners with pagination, search, and filters.
- **FR-008**: Admin MUST be able to edit station details (status, is_live, is_public, name, description, latitude, longitude). Changing coordinates requires explicit confirmation before saving.
- **FR-009**: Admin MUST be able to soft-delete stations from the platform.
- **FR-010**: System MUST support showing/hiding soft-deleted stations via a toggle.
- **FR-011**: Admin MUST be able to view all reviews with station, user, rating, comment, and status.
- **FR-012**: Admin MUST be able to change review status following the lifecycle: submitted → published → flagged → hidden.
- **FR-013**: Admin MUST be able to list all users with email, status, role, and last login.
- **FR-014**: Admin MUST be able to view platform reports (overview KPIs, top stations, search analytics). *(Deferred to Sprint 14/15)*
- **FR-015**: System MUST handle expired auth tokens gracefully (redirect to login).
- **FR-016**: System MUST show loading skeletons during data fetches, not spinners.
- **FR-017**: System MUST handle empty states with helpful guidance messages.
- **FR-018**: System MUST emit clickstream events for admin actions per the event taxonomy.

### Key Entities

- **Partner**: A business entity that owns stations. Attributes: name, type (business/private), status (active/suspended).
- **Station**: A charging station owned by a partner. Attributes: name, status, location, live/public flags, partner.
- **Review**: A user-submitted rating and comment for a station. Attributes: rating (1-5), comment, status lifecycle.
- **User Account**: A platform user with an identity bridged to Keycloak. Attributes: email, status, role.
- **Report Metrics**: Aggregated platform data including station counts, search trends, and activity metrics.

## Success Criteria

### Measurable Outcomes

- **SC-001**: An admin can navigate from login to viewing the dashboard overview in under 3 seconds on a typical connection.
- **SC-002**: An admin can find and edit a specific partner's details in under 5 clicks from the dashboard landing page.
- **SC-003**: An admin can moderate (publish/hide) a review in under 4 clicks from the dashboard.
- **SC-004**: The partner list page renders 20 items in under 2 seconds for a dataset of 100+ partners.
- **SC-005**: Review moderation transitions follow the correct lifecycle — invalid transitions display a clear error.
- **SC-006**: Partner deletion that is blocked by active stations shows which stations are preventing the deletion.

## Assumptions

- Admin API endpoints (`/api/v1/admin/*`) already exist from Sprint 5 (admin-service MVP) and require no backend changes.
- Authentication and RBAC already work from Sprint 3 — admins have the `admin` role in Keycloak.
- The admin-dashboard app scaffold already exists from Sprint 1 monorepo tooling.
- The existing `api-client`, `auth-client`, `api-contracts`, and `design-tokens` packages are reused.
- Design tokens and UI primitives are available from Sprint 8 (Design System Foundation).
- Partners can be searched/filtered by name and status; stations by partner, status, and city.
- Review status transitions follow: submitted → published → flagged → hidden → deleted.
- Reports page is deferred — analytics pipeline must be active before the Reports page is built (Sprint 14+).
- Soft-delete is enforced at the API layer — the frontend never performs hard deletes.
- The admin dashboard targets desktop-first but should be functional on tablets.
- The admin dashboard uses the same 260px sidebar + main content layout as the partner dashboard for navigation consistency.
- Navigation items: Dashboard (overview), Partners, Stations, Reviews, Users.
- Event taxonomy for admin actions (`admin_station.created`, `admin_review.moderated`) already exists from Sprint 13 planning.
