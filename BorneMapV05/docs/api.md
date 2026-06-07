API Specification (v1.0) — Bornemap
1. Purpose

This document defines the complete HTTP API contract for the Bornemap platform.

It covers:

API structure and versioning
Authentication & authorization rules
Resource endpoints (Driver, Partner, Admin)
Pagination, filtering, and sorting
Error model standardization
GIS query patterns
Concurrency rules
Consistency with RBAC + ownership model

This is the single source of truth for backend API design.

2. API Architecture
2.1 Base URL
/api/v1
2.2 Service Boundaries
Service	Base Path
Driver Service	/api/v1/driver/*
Admin Service	/api/v1/admin/*
Partner APIs (Admin Service scoped)	/api/v1/partner/*
Clickstream	internal only (no public API)
GIS Worker	internal only
3. API Design Principles
3.1 REST-only (STRICT)
No GraphQL
No RPC over HTTP
No mixed paradigms
3.2 Stateless
No server session state
JWT-based auth only
3.3 Resource-based naming

✔ Correct:

GET /stations
POST /stations
PATCH /stations/{id}

❌ Incorrect:

/getStations
/createStation
3.4 Versioning
URL versioning only
/api/v1/

No header-based versioning.

4. Authentication
4.1 Mechanism
Authorization: Bearer <JWT>

Issued by Keycloak.

4.2 Roles (STRICT)
registered_driver
partner
admin
4.3 Public Access

No token required.

Public endpoints explicitly defined.

5. Standard Response Format
5.1 Success
{
  "success": true,
  "data": {},
  "meta": {}
}
5.2 Error
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message",
    "details": {}
  }
}
6. Error Codes (Canonical)
Auth
UNAUTHENTICATED
FORBIDDEN
TOKEN_EXPIRED
RBAC / Ownership
PARTNER_SCOPE_VIOLATION
INSUFFICIENT_ROLE
Resource
NOT_FOUND
ALREADY_EXISTS
SOFT_DELETED
Validation
VALIDATION_FAILED
INVALID_COORDINATES
INVALID_STATE_TRANSITION
Business
ACTIVE_STATIONS_EXIST
REVIEW_STATE_INVALID
7. Pagination Model
{
  "page": 1,
  "size": 20,
  "total": 100,
  "total_pages": 5,
  "has_next": true,
  "has_prev": false
}
8. Driver APIs
8.1 Stations Discovery
GET /driver/stations

Query params:

lat
lng
radius_km
bbox
filters
page
size

Rules:

Must use is_live = true
Must exclude deleted stations
Must use GIS index
8.2 Station Detail
GET /driver/stations/{station_id}

Returns:

station info
chargers
availability
reviews summary
8.3 Search Stations
GET /driver/stations/search

Query:

q
city
connector_type
availability
8.4 Favorites (Auth Required)
POST /driver/favorites/{station_id}
DELETE /driver/favorites/{station_id}
8.5 Reviews (Auth Required)
POST /driver/reviews
{
  "station_id": "STN-xxx",
  "rating": 1-5,
  "comment": "text"
}
PATCH /driver/reviews/{id}
DELETE /driver/reviews/{id}

Rules:

Only owner can modify
One review per station per user
8.6 Profile
GET /driver/me
PATCH /driver/me
9. Partner APIs

Base:

/api/v1/partner/*
9.1 Partner Context
GET /partner/me

Returns:

partner_id
role
membership info
9.2 Stations (OWNED ONLY)
GET /partner/stations

Filters:

owned only (MANDATORY)
soft-deleted optional flag
POST /partner/stations

Headers:

Idempotency-Key: required
PATCH /partner/stations/{id}

Triggers:

GIS sync event
analytics event
DELETE /partner/stations/{id}

Soft delete ONLY

9.3 Chargers
GET /partner/chargers
POST /partner/chargers
PATCH /partner/chargers/{id}
9.4 Availability
PATCH /partner/stations/{id}/availability
9.5 Reports
GET /partner/reports/overview
10. Admin APIs

Base:

/api/v1/admin/*
10.1 Users
GET /admin/users
10.2 Partners
GET /admin/partners
POST /admin/partners
PATCH /admin/partners/{id}
DELETE /admin/partners/{id}

Rule:

Cannot delete if active stations exist
10.3 Stations
GET /admin/stations
PATCH /admin/stations/{id}
DELETE /admin/stations/{id}
10.4 Moderation
GET /admin/reviews
PATCH /admin/reviews/{id}/status

States:

published
hidden
flagged
deleted
10.5 Reporting
GET /admin/reports/overview
GET /admin/reports/top-stations
GET /admin/reports/search-analytics
11. GIS Rules (API Level)
11.1 Query Mode

All station queries MUST support:

bbox-based
radius-based
11.2 Response Requirement

Stations MUST include:

{
  "distance_km": 1.2,
  "geom": {
    "lat": 0,
    "lng": 0
  }
}
12. Concurrency Control
Optional optimistic locking
If-Match: <version>

Conflict:

409 CONFLICT
13. Rate Limiting
Public endpoints:
60 req/min
Authenticated:
300 req/min
Admin:
1000 req/min
14. Security Rules
JWT validation required on all protected endpoints
Partner scope ALWAYS enforced server-side
No client-provided partner_id allowed
Public endpoints explicitly declared
No business logic in frontend
15. Event Integration

API MUST emit:

Station lifecycle:
station.created
station.updated
station.deleted
User actions:
favorite_station.added
review.submitted
Partner actions:
partner_station.updated
16. Performance Constraints
Station search ≤ 200ms (p95)
GIS queries MUST use spatial index
Pagination mandatory for all list endpoints
No full table scans allowed
17. Versioning Rules
Breaking changes → /v2
Non-breaking → /v1

No header versioning allowed.

18. Non-Negotiable Rules
Partner isolation enforced in every query
Soft delete only (no hard delete APIs exposed except admin override)
GIS is read-optimized only
Analytics never exposed via API
Public endpoints must be safe for caching
19. Summary

This API spec enforces:

strict REST design
RBAC + tenant isolation
GIS-first querying model
event-driven integration
consistent response contracts
scalable public + admin + partner separation
