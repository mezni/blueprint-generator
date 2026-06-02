ck (2–3 days)
Goal

Freeze system boundaries before coding anything.

Deliverables
constitution.md (final)
architecture.md (final)
domain-model.md (draft v1)
event-taxonomy.md (v1 locked)
api-spec.md (empty but structured)
database-spec.md (logical schema only)
deployment-model.md
Exit Criteria
All services defined
No ambiguity in ownership (Keycloak vs platform_db)
Event envelope finalized
ID strategy finalized (USR-, STN-, REV-)
🏗️ Phase 1 — Monorepo Foundation (Week 1–2)
Goal

Create engineering skeleton + shared contracts.

Work Items
Repo Structure
apps/
services/
packages/
infra/
docs/
Initialize
Rust workspace
React + Vite apps (3 dashboards + driver web)
Expo RN app
Shared TS packages
Event taxonomy package
API contracts package
Core Packages
common-types
common-auth
api-client
design-tokens (empty first)
event-taxonomy (locked schema)
Exit Criteria
All apps compile
Empty UI shells running
Shared types imported cross-stack
⚙️ Phase 2 — Runtime Foundation (Week 3–4)
Goal

Bring full system online locally via Docker Compose.

Services
Infrastructure
Traefik
Keycloak
PostgreSQL (3 DBs)
RabbitMQ
Backend Services
driver-service (empty API)
admin-service (empty API)
clickstream-service
gis-worker
analytics-writer
Rules enforced
Only Traefik is public
Everything internal network only
Env-driven configuration only
Exit Criteria
docker compose up brings full system online
Keycloak issues JWT
DB connectivity validated
RabbitMQ reachable
🔐 Phase 3 — Identity & RBAC (Week 4–5)
Goal

Authentication + authorization backbone

Work Items
Keycloak realm setup
Roles:
registered_driver
partner
admin
JWT middleware (Rust)
First-login provisioning
users.user_account creation
partner_membership mapping
Critical Rule Implemented
partner_id NEVER comes from client
Exit Criteria
Login works
JWT validated in services
Role-based endpoint protection active
🗄️ Phase 4 — Database Foundation (Week 5–6)
Goal

Stable data model before business logic

Schema Build
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
analytics
raw_event (partitioned)
aggregates
GIS
OSM base layers
station geometry layer
sync_queue
Rules enforced
soft delete everywhere
station visibility rule enforced
GIST indexing for geom
Exit Criteria
migrations stable
seed data works
spatial queries functional
🛠️ Phase 5 — Admin Service MVP (Week 6–8)
Goal

Inventory write system (backend only)

Features
partner CRUD
station CRUD
charger CRUD
availability update
Hard Rules
full partner isolation
no cross-partner queries
GIS outbox trigger on station change
Exit Criteria
Admin can fully manage inventory
DB consistency enforced
RBAC fully working
🌍 Phase 6 — GIS System (Week 8–10)
Goal

Spatial intelligence layer

Components
OSM import (Tunisia first)
GIS Sync Worker
Outbox processor
Replay system
States
pending
processing
succeeded
failed
dead-letter
superseded
Exit Criteria
station → geometry sync works
map queries functional
idempotency verified
🚗 Phase 7 — Driver Backend (Week 10–12)
Goal

User-facing API

Features
Public
station discovery
bbox search
station detail
map markers
Authenticated
favorites
reviews
profile
Rules
is_live only
soft delete excluded
Tunisia fallback center
Exit Criteria
full discovery working
mobile-ready APIs stable
🎨 Phase 8 — Design System (Week 12–13)
Goal

UI foundation before frontend scale

Deliverables
design-tokens (final)
shadcn/ui layer
Tailwind theme
component primitives
Exit Criteria
reusable UI system exists
RTL supported
tokens enforced
🌐 Phase 9 — Web Applications (Week 13–16)
Apps
Driver Web App
Partner Dashboard
Admin Dashboard
Stack
React + Vite
React Query
Leaflet
shared design system
Exit Criteria
all dashboards functional
map fully integrated
RBAC UI enforced
📱 Phase 10 — Mobile App (Week 16–18)
Features
station discovery
map UX
favorites
reviews
login
Exit Criteria
parity with driver web core features
RTL stable
offline-safe UI behavior
📡 Phase 11 — Clickstream + Analytics (Week 18–20)
Goal

Event-driven intelligence

Components
clickstream-service
RabbitMQ pipeline
analytics-writer
Rules
event_id deduplication
strict taxonomy validation
no PII in payload
Exit Criteria
events flowing end-to-end
analytics DB populated
📊 Phase 12 — Reporting (Week 20–21)
Partner
station performance
engagement metrics
Admin
system KPIs
moderation stats
top stations
🧪 Phase 13 — Hardening (Week 21–23)
Goal

Production readiness

Includes
load testing (<100 events/sec baseline)
RBAC audit
GIS consistency checks
RTL validation
WCAG 2.1 AA audit
failure simulation
rollback drills
🚀 Final System Outcome

You will have:

Core backend
Rust microservices (minimal set)
Keycloak identity
PostgreSQL (3 DBs)
RabbitMQ event backbone
Intelligence layer
GIS sync system
event-driven analytics
clickstream pipeline
Products
Driver Web App
Driver Mobile App
Partner Dashboard
Admin Dashboard
Guarantees
strict partner isolation
event integrity
GIS correctness
deterministic deployment via Docker Compose
