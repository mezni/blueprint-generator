Bornemap Domain Model Specification (v1.0)
1. Purpose

This document defines the business domain model of Bornemap.

It specifies:

domain entities and their meaning
invariants (business rules that must never be violated)
lifecycle rules
cross-entity relationships
behavioral constraints (what the system is allowed to do)
ownership boundaries (partner isolation, user identity, etc.)

This document is the source of truth for business logic, independent of APIs, UI, or infrastructure.

2. Core Domain Overview

The system is composed of 5 core domains:

Identity Domain (Keycloak)
        ↓
User Domain (platform_db.users)
        ↓
Inventory Domain (Stations / Chargers)
        ↓
Interaction Domain (Favorites / Reviews)
        ↓
Spatial Domain (GIS)
        ↓
Analytics Domain (Events)

Each domain is strictly separated.

3. Identity Domain (External Authority)
3.1 System Owner

Keycloak

3.2 Responsibilities
Authentication
Login / logout
OAuth providers
JWT issuance
Role assignment
3.3 Domain Rule (Critical)

Identity is NOT part of the application database.

The platform MUST NOT:

store passwords
store sessions
replicate identity state
3.4 Roles

Only:

registered_driver
partner
admin
4. User Domain (Application Identity Layer)

Stored in platform_db.users

4.1 user_account (Identity Bridge)
Meaning:

Represents a platform identity linked to Keycloak.

Fields:
id (USR-*)
keycloak_user_id (unique)
email
status
last_login_at
Invariants:
MUST map 1:1 with Keycloak user
MUST NOT contain business ownership logic
4.2 user_profile
Meaning:

User preferences and UI personalization.

Fields:
display_name
avatar
preferred_language
preferences JSON
Invariants:
optional data
safe to delete without breaking system logic
4.3 partner_membership (CRITICAL DOMAIN ENTITY)
Meaning:

Defines ownership of a user within a partner organization.

Fields:
user_id (unique)
partner_id (required)
role: owner | manager | operator | viewer
Invariants:
A user can belong to ONLY ONE partner
partner_id MUST always exist if role = partner
cannot be null for partner users
Business Rule:

Partner identity is derived ONLY from membership, never from JWT alone.

5. Inventory Domain (Core Business Asset Domain)

This is the most important domain in the system.

5.1 Partner Entity
Meaning:

Represents a charging operator organization.

Key fields:
id (PRT-*)
name
status
type (business/private)
deleted_at (soft delete)
Invariants:
Partner can exist without stations
Partner MUST be soft deleted only
Partner deletion is blocked if active stations exist
5.2 Station Entity (CORE ENTITY)
Meaning:

A physical EV charging location.

Key fields:
id (STN-*)
partner_id (owner)
name
geom (lat/lng → PostGIS)
status: active | inactive | draft
is_live (boolean)
is_public (boolean)
deleted_at
Invariants (CRITICAL):
MUST belong to exactly one partner
MUST have valid geometry
MUST NOT be visible if:
deleted_at IS NOT NULL
is_live = false
status != active
Lifecycle Rules:
draft → active → inactive → deleted
Side Effects:

Any change triggers:

GIS sync event
analytics event
cache invalidation
5.3 Charger Entity
Meaning:

Physical charging unit inside a station.

Key fields:
id (CHG-*)
station_id
connector_type
power_kw
status
Invariants:
MUST belong to a station
inherits partner ownership through station
cannot exist without station
5.4 Station Availability (Derived Operational State)
Meaning:

Current operational availability snapshot.

Fields:
station_id
status: available | limited | unavailable
updated_by
update_source (manual_partner | system)
Rule:

This is a mutable operational projection, not a source of truth.

6. Interaction Domain
6.1 Favorite Station
Meaning:

User bookmark system.

Invariants:
user_id + station_id unique pair
only registered_driver can create
6.2 Station Review
Meaning:

User-generated rating + comment.

Fields:
id (REV-*)
user_id
station_id
rating (1–5)
comment
status (published | hidden | flagged | deleted)
Invariants:
one review per user per station
only owner can modify
admin controls moderation state only
Lifecycle:
submitted → published → flagged → hidden → deleted
7. Spatial Domain (GIS)
7.1 Nature of Domain

GIS is:

derived
asynchronous
eventually consistent

NOT authoritative.

7.2 Station Geometry Rule
geom = ST_SetSRID(Point(long, lat), 4326)
7.3 GIS State Invariants
every station MUST have a geometry OR fallback location
updates MUST be idempotent
sync MUST be replay-safe
7.4 GIS Lifecycle
station_change → outbox → queue → worker → GIS update → enriched view
8. Analytics Domain
8.1 Nature
append-only
event-driven
eventually consistent
8.2 Event Entity
Meaning:

A user/system interaction record.

Required fields:
event_id (unique)
event_name
session_id
actor_type
timestamp
payload (JSONB)
8.3 Invariants:
MUST be immutable
MUST support deduplication
MUST tolerate replay
9. Cross-Domain Invariants (CRITICAL RULES)
9.1 Ownership Rule

Every mutable business entity MUST have exactly one owner.

Station → Partner
Review → User
Charger → Station → Partner
9.2 No Cross-Partner Leakage

A partner MUST NEVER:

read other partner data
modify other partner stations
infer other partner analytics
9.3 Soft Delete Rule

Entities:

Partner
Station

MUST:

use soft delete only
never hard delete in production
9.4 Event Side Effect Rule

Any of the following MUST emit events:

station created/updated/deleted
review created/updated/deleted
availability changed
9.5 GIS Consistency Rule

GIS must ALWAYS reflect:

latest station state
eventual consistency allowed
never block business operations
10. Domain Interaction Map
Identity (Keycloak)
        ↓
User Domain (platform_db)
        ↓
Partner Ownership Resolution
        ↓
Inventory Domain (Stations/Chargers)
        ↓
Interaction Domain (Favorites/Reviews)
        ↓
GIS Domain (Derived Map State)
        ↓
Analytics Domain (Events)
11. Business Rules Summary (CORE TRUTH)
Station
owned by one partner
visible only if active + live
drives GIS updates
Partner
strict isolation boundary
one membership per user
User
identity comes from Keycloak
profile is optional
Review
one per user per station
moderated by admin only
GIS
derived only
never authoritative
Analytics
append-only
never used for decisions
12. Final System Principle

The domain model is the source of truth for all business behavior.
Everything else (API, UI, events, GIS, analytics) is a projection of it.
