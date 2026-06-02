Bornemap Architecture (v1.0)
1. Purpose

This document defines the system architecture of Bornemap, including:

service topology
data flow architecture
domain boundaries
communication patterns
runtime deployment model
eventing + GIS pipelines
frontend/backend integration model

It is the technical blueprint derived from the Constitution.

2. High-Level System Architecture
2.1 Core Layers

The system is structured into 5 layers:

[ Frontend Layer ]
        ↓
[ API Layer (REST Services) ]
        ↓
[ Domain Services Layer ]
        ↓
[ Event + Processing Layer ]
        ↓
[ Data Layer (PostgreSQL + GIS + Analytics) ]
2.2 Physical Deployment Model

All components run on:

Bare metal / VM
Docker Compose
Traefik as ingress
Public exposure rule:

Only Traefik is exposed to the internet.

3. System Components
3.1 Frontend Applications
Driver Applications
Driver Web (React + Vite)
Driver Mobile (React Native Expo)

Capabilities:

station discovery
map interaction
favorites
reviews
profile
Dashboard Applications
Partner Dashboard
Admin Dashboard

Capabilities:

operational control
station management
moderation
reporting
3.2 Backend Services
Core Services
Driver Service

Responsibilities:

station discovery APIs
search + filters
user interactions (favorites, reviews)
public + authenticated endpoints
Admin Service

Responsibilities:

partner management
station CRUD
charger CRUD
moderation
reporting APIs
Clickstream Service

Responsibilities:

event ingestion (RabbitMQ consumer)
validation of event taxonomy
forwarding to analytics pipeline
GIS Worker

Responsibilities:

spatial enrichment
station geometry processing
outbox consumption
synchronization to GIS layer
Analytics Writer

Responsibilities:

event aggregation
partitioned storage in analytics_db
KPI computation base layer
4. Data Architecture
4.1 Databases
keycloak_db
Identity system only
managed by Keycloak
no application logic
platform_db (Core Business DB)

Schemas:

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
derived spatial structures only
analytics_db
raw_event (partitioned)
aggregates
optional derived metrics
4.2 Data Ownership Rules
Domain	Owner
Identity	Keycloak
Users	platform_db
Stations	platform_db
GIS	GIS Worker (derived)
Analytics	analytics_db
5. Communication Architecture
5.1 API Communication

All services expose:

Pure REST APIs
JSON request/response
Standard envelope format

No:

GraphQL
RPC frameworks
internal service mesh complexity
5.2 Event Communication (RabbitMQ)

Used for:

station lifecycle events
GIS sync triggers
clickstream ingestion
analytics pipeline
5.3 Event Flow Model
Frontend Action
      ↓
Backend Service (REST)
      ↓
Outbox Table (DB)
      ↓
RabbitMQ Queue
      ↓
Consumer Service
      ↓
Target System (GIS / Analytics)
6. Domain Architecture
6.1 Station Domain (Core Entity)

Station lifecycle:

CREATE → UPDATE → PUBLISH → DELETE (soft)

Rules:

station is owned by partner
station is hidden unless is_live = true
GIS sync triggered on all mutations
6.2 Partner Domain
strict ownership model
one user → one partner membership
all queries scoped by partner_id
6.3 User Domain
Keycloak = identity
platform_db = application profile
no duplication of credentials
6.4 Review Domain

Rules:

user owns review
station owns aggregation
admin can moderate state only
7. GIS Architecture
7.1 GIS Pipeline
Station Change
     ↓
Outbox Event
     ↓
RabbitMQ
     ↓
GIS Worker
     ↓
GIS Layer Update
     ↓
Driver Service Query Enrichment
7.2 GIS Model

GIS is:

derived
asynchronous
eventually consistent

NOT authoritative.

7.3 Consistency Model
eventual consistency
idempotent processing
replay-safe workers
8. Clickstream Architecture
8.1 Event Pipeline
Frontend Event
     ↓
Clickstream Service
     ↓
RabbitMQ
     ↓
Analytics Writer
     ↓
analytics_db
8.2 Event Rules
event_id required (deduplication)
session_id required
actor optional for anonymous users
payload is schema-flexible JSONB
8.3 Guarantee
at-least-once delivery
consumer-side deduplication mandatory
9. API Architecture
9.1 REST Model

Standard:

/api/v1/{domain}/{resource}

Examples:

/api/v1/driver/stations
/api/v1/admin/partners
/api/v1/partner/stations
9.2 Response Contract

Success:

{
  "success": true,
  "data": {},
  "meta": {}
}

Error:

{
  "success": false,
  "error": {
    "code": "STRING",
    "message": "STRING"
  }
}
10. Frontend Architecture
10.1 Shared System

All frontend apps share:

design tokens
API client
auth client
shared types
10.2 Driver UX Model

Key principle:

Map-first experience

Flow:

Map Load → Viewport Query → Markers → Station Detail
10.3 Dashboard UX Model

Key principle:

Density-first operational UI

tables
filters
KPI cards
management panels
11. Authentication Architecture
11.1 Identity Flow
User → Keycloak → JWT → Backend Validation
11.2 Authorization Flow
JWT → Role Check → Ownership Check → Repository Filter
11.3 Roles
registered_driver
partner
admin
12. Deployment Architecture
12.1 Runtime Model
Docker Compose
VM / bare metal
Traefik ingress
12.2 Service Network

Internal-only communication:

driver-service.internal
admin-service.internal
clickstream.internal
gis.internal
analytics.internal
12.3 Public Exposure

Only:

Traefik
Keycloak (auth endpoint)
13. Observability Architecture
13.1 Logging
structured JSON logs
request_id propagation
service_name tagging
13.2 Metrics

Minimum:

API latency
DB query time
queue depth
GIS lag
event ingestion rate
13.3 Tracing
correlation_id across services
event_id traceability in analytics pipeline
14. Failure Model
14.1 Failure Domains
DB failure → service degraded
RabbitMQ backlog → delayed analytics/GIS
GIS worker failure → eventual sync delay
Keycloak failure → auth outage
14.2 Resilience Strategy
retry with backoff
idempotent processing
queue replay capability
soft failure in GIS/analytics
15. Performance Model

System designed for:

< 100 events/sec
moderate map traffic
single-region deployment
no horizontal scaling required initially
16. Key Architectural Principles (Final)
16.1 Data-first architecture

Business data is authoritative, not UI or events.

16.2 Async where needed, sync where required
GIS → async
analytics → async
auth → sync
station CRUD → sync
16.3 Strict ownership enforcement

Partner isolation is enforced at backend + repository level.

16.4 No distributed overengineering

No:

service mesh
event sourcing core
CQRS complexity (beyond GIS/analytics separation)
17. Final System Summary

Bornemap is:

A REST-based, map-centric EV station platform with strict identity separation (Keycloak), transactional core data (PostgreSQL), asynchronous enrichment (GIS), and behavioral analytics (clickstream), deployed on a minimal bare-metal Docker Compose infrastructure with Traefik ingress.
