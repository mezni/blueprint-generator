Bornemap Constitution (v1.0 — Reset)
1. Purpose

This document defines the canonical architecture, domain boundaries, and system rules for the Bornemap EV station platform.

It is the highest authority for:

system structure
service boundaries
data ownership
identity model
eventing model
GIS behavior
authorization rules
deployment constraints

It overrides all implementation documents.

2. System Scope

The platform provides EV charging station discovery and management with three user domains:

2.1 Driver (Public + Registered)
Station discovery (map + search)
Station details
Favorites (registered only)
Reviews (registered only)
Profile management (registered only)
2.2 Partner
Own station management
Charger management
Availability updates
Partner-scoped reporting
2.3 Admin
Global platform control
User management
Partner management
Station moderation
System reporting
3. Core Architecture Principles
3.1 Pragmatic Monolith-of-Services

The system uses a small set of independent services, not microservice fragmentation.

Services are independent but tightly governed.

3.2 Strict Separation of Concerns

The system is divided into:

Identity (Keycloak)
Business Data (platform_db)
Spatial Data (GIS layer)
Analytics (analytics_db)
Events (RabbitMQ clickstream pipeline)

No overlap is allowed between domains.

3.3 Source of Truth Rule
Domain	Source of Truth
Identity	Keycloak
Users	platform_db
Stations	inventory schema
GIS data	GIS layer (derived)
Analytics	analytics_db
Events	Clickstream service
3.4 No Cross-Domain Leakage
Analytics cannot affect business logic
GIS is derived, never authoritative
Identity is never duplicated in DB
Authorization is never inferred from frontend
4. Identity Model
4.1 Identity Provider
Keycloak is the only authentication system

It handles:

login/logout
OAuth providers
JWT issuance
session lifecycle
role assignment
4.2 Roles (Strict Set)

Only three roles exist:

registered_driver
partner
admin

No additional roles are allowed.

4.3 Public Access

Public users:

are anonymous
are NOT stored in DB
are NOT Keycloak users
are NOT a role
4.4 Identity Mapping Rule
platform_db.users.user_account.keycloak_user_id = JWT.sub

This is the only identity bridge.

5. Authorization Model
5.1 Enforcement Layers

Authorization MUST be enforced at:

Backend (primary enforcement)
Repository/data access layer (mandatory)
Frontend (UX only, non-secure)
5.2 Partner Isolation Rule (Critical)

Partners can only access their own data:

stations
chargers
availability
reports

Enforced via:

users.partner_membership.partner_id

No client-supplied tenant IDs allowed.

5.3 Admin Scope

Admins have global access but:

must still respect data integrity rules
cannot bypass audit logging
cannot bypass soft delete rules
6. Data Architecture
6.1 Databases

The system uses three PostgreSQL databases:

keycloak_db → identity only
platform_db → business data
analytics_db → event analytics
6.2 platform_db Schemas
inventory
partner
station
charger
station_availability
users
user_account
user_profile
partner_membership
favorite_station
station_review
gis
derived spatial data only
never authoritative
6.3 Soft Delete Rule

Entities:

station
partner

MUST use soft delete only.

6.4 Station Visibility Rule

A station is visible if:

is_live = true
deleted_at IS NULL
status = active
is_public = true
7. GIS System Rules
7.1 GIS is Derived Data

GIS is NOT a source of truth.

It is derived from:

station geometry
station metadata
7.2 Sync Model

All updates go through:

outbox table
RabbitMQ queue
GIS worker
7.3 Idempotency Rule

All GIS updates must be:

idempotent
replay-safe
version-controlled
8. Eventing System (Clickstream)
8.1 Transport
RabbitMQ is the event backbone
8.2 Event Model

All events must follow:

strict envelope
event_id deduplication
session tracking
actor attribution
8.3 Event Scope

Only:

user behavior events
system interaction events
UI interaction events

NOT:

business state source of truth
authorization decisions
identity logic
8.4 Delivery Guarantee
at-least-once delivery
consumers must deduplicate
9. API Model
9.1 Architecture Style
Pure REST APIs only
No GraphQL
No BFF complexity
9.2 Standard Envelope

Success:

{ "success": true, "data": {}, "meta": {} }

Error:

{ "success": false, "error": { "code": "", "message": "" } }
10. Service Architecture
10.1 Core Services
driver-service
admin-service
clickstream-service
gis-worker
analytics-writer
10.2 Responsibility Rule
Service	Responsibility
Driver Service	read + user actions
Admin Service	system mutation
GIS Worker	spatial sync
Clickstream	event ingestion
Analytics	aggregation
11. Frontend Architecture
11.1 Applications
Driver Web App
Driver Mobile App
Partner Dashboard
Admin Dashboard
11.2 Shared System
design tokens (single source)
design system (shared components)
API client (shared contracts)
11.3 UX Rule
mobile-first for drivers
dashboards optimized for density
public browsing without friction
progressive authentication only
12. Infrastructure Model
12.1 Deployment
Bare metal or VM
Docker Compose
Traefik ingress
12.2 External Exposure Rule

Only Traefik is public.

Everything else is internal-only.

12.3 No Registry Dependency

Images are:

locally loaded
or shipped as artifacts
13. Configuration Model
environment variables only
host-managed secrets
no runtime secret generation
fail-fast on invalid config
14. Observability (Baseline Rule)

All services must emit:

structured logs
request correlation IDs
basic metrics
error tracing metadata
15. Performance Assumptions

System is designed for:

< 100 events/sec
moderate concurrent users
single-region deployment
no horizontal scaling complexity initially
16. Security Model

Mandatory rules:

Keycloak is only auth system
JWT required for all protected endpoints
no DB-stored credentials
strict partner isolation
repository-layer enforcement required
17. Testing Principle

Testing must validate:

authorization correctness
GIS correctness
event integrity
API contracts
soft delete behavior
partner isolation
18. Final System Principle

The system is not event-driven, not GIS-driven, and not UI-driven.

It is:

👉 Data-first, contract-driven, and ownership-enforced.
