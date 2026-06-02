Database Specification (v1.0) — Bornemap
1. Purpose

This document defines the canonical database architecture and data modeling rules for the Bornemap platform.

It covers:

Physical database topology
Schema ownership boundaries
Domain models (inventory, users, GIS, analytics)
Constraints and indexing strategy
Soft delete and audit rules
Event-driven consistency assumptions
GIS and spatial data rules

It is the single source of truth for data design.

2. Database Topology

The system uses three PostgreSQL databases:

2.1 keycloak_db (Identity Only)

Owned exclusively by Keycloak.

Contains:

users
credentials
sessions
realms
roles

⚠️ No business data allowed.

2.2 platform_db (Core Business Data)

Schemas:

inventory
users
gis

This is the system of record for all operational data.

2.3 analytics_db (Analytical Data)

Schema:

analytics

Contains:

event logs
aggregated metrics
derived insights

⚠️ Never used for operational truth.

3. Source of Truth Rules
Domain	Source
Identity	Keycloak
Users	platform_db.users
Stations	platform_db.inventory.station
Chargers	platform_db.inventory.charger
Reviews	platform_db.users.station_review
Favorites	platform_db.users.favorite_station
GIS geometry	platform_db.inventory.station.geom
Events	Clickstream → analytics_db.analytics
4. Global Design Principles
4.1 Soft Delete Everywhere

Entities:

station
partner
review (logical delete only)

Rule:

deleted_at IS NULL
4.2 ID Strategy (ULID + Prefix)
Entity	Format
User	USR-ULID
Partner	PRT-ULID
Station	STN-ULID
Charger	CHG-ULID
Review	REV-ULID
Event	EVT-ULID
4.3 Audit Fields (Mandatory)

All mutable entities MUST include:

created_at
updated_at
created_by
updated_by
deleted_at
5. Schema: inventory
5.1 inventory.partner

Represents EV infrastructure operators.

id (PRT-*)
name
type (business | private)
status (active | suspended)
created_at
updated_at
deleted_at

Indexes:

BTREE(id)
BTREE(status)
5.2 inventory.station (CORE ENTITY)

Canonical EV station.

id (STN-*)
partner_id FK → inventory.partner.id

name
description

latitude
longitude
geom GEOGRAPHY(Point, 4326)

status (active | inactive | maintenance)
is_live BOOLEAN
is_public BOOLEAN

city
country

created_at
updated_at
deleted_at
Constraints:
Partner ownership enforced
geom MUST match lat/lng
Indexes:
GIST (geom)
BTREE(partner_id)
BTREE(status)
BTREE(is_live, is_public)
BTREE(city)
5.3 inventory.charger
id (CHG-*)
station_id FK → station

type (CCS | Type2 | CHAdeMO)
power_kw
status (available | offline | fault)

created_at
updated_at
deleted_at

Indexes:

BTREE(station_id)
BTREE(status)
5.4 inventory.station_availability
id
station_id
status (available | limited | unavailable)
source (manual_partner | system_sync | admin)

updated_at

Index:

BTREE(station_id)
6. Schema: users
6.1 users.user_account

Bridge between Keycloak and platform.

id (USR-*)
keycloak_user_id UNIQUE

email
status (active | disabled)

created_at
last_login_at

Index:

UNIQUE(keycloak_user_id)
6.2 users.user_profile
user_id FK → user_account

display_name
avatar_url
preferred_language
preferences JSONB
6.3 users.partner_membership

STRICT 1:1 mapping.

user_id UNIQUE FK → user_account
partner_id FK → inventory.partner

role (owner | manager | operator | viewer)

Constraint:

UNIQUE(user_id)
6.4 users.favorite_station
user_id
station_id
created_at

PK:

(user_id, station_id)
6.5 users.station_review
id (REV-*)
user_id
station_id

rating (1–5)
comment TEXT

status (published | hidden | flagged | deleted)

created_at
updated_at

Constraints:

UNIQUE(user_id, station_id)

Indexes:

BTREE(station_id)
BTREE(user_id)
7. Schema: gis
7.1 gis.sync_queue

Outbox-driven GIS sync system.

id
entity_type (station | charger)
entity_id
operation (insert | update | delete)

payload JSONB

status (pending | processing | done | failed | dead_letter)

created_at
processed_at

Index:

BTREE(status)
BTREE(entity_type, entity_id)
8. Schema: analytics
8.1 analytics.raw_event

Partitioned by month.

event_id
event_name

session_id
user_id
anonymous_id

actor_role

occurred_at
ingested_at

path

payload JSONB
metadata JSONB

Indexes:

BTREE(event_name, occurred_at)
BTREE(user_id)
BTREE(session_id)
8.2 analytics.event_dead_letter

Stores invalid events.

9. GIS Rules (CRITICAL)
9.1 Geometry is authoritative
geom = ST_SetSRID(ST_MakePoint(lng, lat), 4326)
9.2 Visibility Rule

A station is visible if:

is_live = true
AND deleted_at IS NULL
AND status = 'active'
AND is_public = true
10. Event-Driven Consistency
10.1 Source events trigger:
station.created
station.updated
station.deleted
10.2 Consumers:
GIS Worker → updates geometry/state
Analytics → stores event
Admin → audit log
11. Soft Delete Policy
NEVER hard delete stations
NEVER hard delete partners
Reviews are logically deleted only
12. Indexing Strategy
Required indexes:
All foreign keys
All GIS geometry fields (GIST)
All query filters:
status
partner_id
is_live
station_id
13. Performance Rules
bbox queries MUST use GIST index
station discovery MUST be paginated
analytics writes MUST be append-only
GIS sync MUST be async
14. Data Ownership Matrix
Domain	Owner
Identity	Keycloak
Users	Platform DB
Stations	Inventory
GIS	GIS Worker (derived)
Events	Clickstream
Analytics	Analytics DB
15. Non-Negotiable Rules
No cross-schema writes without service boundary
No direct analytics dependency in runtime logic
No client-provided ownership fields
Partner isolation enforced in query layer
GIS always derived, never source of truth
16. Summary

This database model enforces:

strict domain separation
GIS-first spatial consistency
event-driven synchronization
strong partner isolation
analytics decoupling
scalable PostgreSQL architecture
