BornemapV1 — Sprint Breakdown (Granular)
Assumption
Sprint length: 1 week
Solo developer velocity
No parallel teams
Strict dependency gating
🟦 Sprint 0 — Architecture Freeze (Week 0)
Goal

Lock the system so implementation cannot drift.

Tasks
Finalize constitution.md
Finalize architecture.md
Finalize:
domain model (v1)
event taxonomy (v1 locked)
API envelope standard
ID strategy (USR-, STN-, REV-)
Define service list (no more changes allowed)
Outputs
/docs/*.md baseline frozen
empty repo initialized
Exit Criteria
No open design questions
All boundaries explicit
🟦 Sprint 1 — Monorepo + Tooling Foundation
Goal

Create full engineering workspace

Backend
Rust workspace initialized
shared crates:
common-types
common-errors
common-auth (stub)
common-db (stub)
Frontend
React + Vite apps:
driver-web
partner-dashboard
admin-dashboard
React Native Expo app scaffold
Shared
event-taxonomy package
api-contracts package
Infra
empty Docker Compose structure
base Traefik config
Exit Criteria
all apps compile
empty screens render
Rust workspace builds
🟦 Sprint 2 — Runtime Infrastructure (Docker Compose v1)
Goal

Bring system alive locally

Infrastructure
PostgreSQL (3 DBs)
Keycloak
RabbitMQ
Traefik
Backend Services (empty HTTP)
driver-service
admin-service
clickstream-service
gis-worker
analytics-writer
Tasks
docker compose wiring
internal networking
env system per service
Traefik routing rules
Exit Criteria
docker compose up works end-to-end
Keycloak reachable
services respond /health
🟦 Sprint 3 — Identity & Auth Foundation
Goal

Authentication + RBAC backbone

Keycloak
realm: ev-platform
roles:
registered_driver
partner
admin
Backend
JWT validation middleware (Rust)
role extraction layer
auth guard framework
DB
users.user_account created
first-login provisioning logic
Rules implemented
no DB-based auth
Keycloak is source of truth
Exit Criteria
protected endpoints enforce roles
JWT validated everywhere
🟦 Sprint 4 — Core Database Schema (Inventory + Users)
Goal

Define system truth before business logic

Inventory schema
partner
station
charger
station_availability
User schema
user_account
user_profile
partner_membership
favorite_station
station_review
Analytics schema
raw_event (partitioned stub)
GIS schema
station_geom table
sync_queue
Exit Criteria
migrations run clean
relationships enforced
indexes created
🟦 Sprint 5 — Admin Service MVP (Inventory Write API)
Goal

First real business system

Features
partner CRUD
station CRUD
charger CRUD
availability updates
Rules enforced
partner isolation mandatory
soft delete enabled
no analytics logic
Critical
station change triggers GIS outbox event
Exit Criteria
admin can fully manage inventory
data persists correctly
🟦 Sprint 6 — GIS Sync System v1
Goal

Spatial system operational

Components
GIS Worker
Outbox table processor
station → geometry sync
Features
OSM Tunisia import (basic)
station geo conversion
sync_queue processing
States
pending → processing → success/fail
Exit Criteria
station updates reflected in GIS layer
idempotent processing verified
🟦 Sprint 7 — Driver Service MVP (Discovery Core)
Goal

Public station discovery system

Public APIs
station list (bbox)
station detail
map markers
search
Authenticated APIs
favorites
reviews
profile
Rules
is_live enforced
deleted excluded
Tunis fallback center
Exit Criteria
full map flow works
bbox queries performant
🟦 Sprint 8 — Design System Foundation
Goal

UI consistency layer

Deliverables
Tailwind theme (tokens applied)
shadcn/ui integration
design-tokens package finalized
Components v1
Button
Input
Card
Modal
Map container shell
Exit Criteria
all apps use same styling base
RTL ready foundation exists
🟦 Sprint 9 — Driver Web App
Goal

First full UX product

Features
map discovery
station detail
search/filter
favorites
reviews
Exit Criteria
full driver journey works
mobile-style UX mirrored in web
🟦 Sprint 10 — Partner Dashboard
Goal

Operational dashboard

Features
station management
charger management
availability updates
Exit Criteria
partner isolation verified
CRUD fully functional
🟦 Sprint 11 — Admin Dashboard
Goal

System control interface

Features
global station view
partner management
moderation (basic)
system overview
Exit Criteria
admin has full platform control
RBAC enforced
🟦 Sprint 12 — Mobile App (Expo)
Goal

Driver mobile experience

Features
map discovery
station details
favorites
reviews
login flow
Exit Criteria
parity with driver web core
smooth map UX on mobile
🟦 Sprint 13 — Clickstream System
Goal

Event-driven analytics backbone

Components
clickstream-service
RabbitMQ pipeline
event validation
Features
event ingestion
deduplication (event_id)
validation against taxonomy
Exit Criteria
events flowing from frontend → DB
🟦 Sprint 14 — Analytics Writer
Goal

Turn events into usable data

Features
raw_event ingestion
partitioning
aggregation pipeline
Exit Criteria
analytics DB populated
queries operational
🟦 Sprint 15 — Reporting Layer
Goal

Business insights

Partner
station performance
usage metrics
Admin
platform KPIs
top stations
search trends
🟦 Sprint 16 — Hardening + Production Readiness
Goal

Make system production-safe

Work
load testing (<100 events/sec baseline)
RBAC audit
GIS stress testing
retry & failure simulation
rollback drills
DB backup validation
UX
RTL audit (Arabic)
WCAG 2.1 AA check
mobile weak network testing
Exit Criteria
production-ready deployment checklist passes
📊 Sprint Dependency Graph (Simplified)
0 → 1 → 2 → 3 → 4
            ↓
            5 → 6 → 7
                     ↓
          8 → 9 → 10 → 11 → 12
                              ↓
                          13 → 14 → 15
                                    ↓
                                   16
🧠 Execution Reality Check

This plan enforces:

Correct ordering
identity before data access
data before GIS
GIS before driver UX
UX before analytics
Risk control
no early UI explosion
no premature analytics
no coupling between services
Solo dev optimization
every sprint produces usable system state
no “invisible progress” phases
