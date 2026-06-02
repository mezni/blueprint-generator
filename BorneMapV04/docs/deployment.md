Deployment Model (Bornemap v1.0)
1. Purpose

This document defines the production and staging deployment architecture for the Bornemap platform.

It specifies:

infrastructure topology
runtime model
service orchestration
networking and ingress
artifact delivery model
scaling constraints
operational boundaries

It is the authoritative reference for how the system runs in production.

2. Deployment Philosophy

The platform is designed for:

bare metal simplicity
deterministic runtime behavior
manual operational control
container-based isolation without Kubernetes complexity
Core principles
No Kubernetes
No service mesh
No cloud lock-in
No registry dependency (artifact-based deployment)
Single ingress controller (Traefik)
Explicit service boundaries
3. Infrastructure Topology
3.1 Physical/VM Layout

Supported models:

Option A — Single Node (MVP / Dev)
all services on one VM
Docker Compose stack
Option B — Multi-node (Production)
1 ingress node (Traefik)
1 backend node (services)
1 data node (PostgreSQL + RabbitMQ)
3.2 Network Zones
[ Public Internet ]
        |
     Traefik
        |
------------------------------------------------
| Internal Docker Network (NO public exposure) |
------------------------------------------------
   |        |         |          |
Keycloak  Services   Workers   Databases
4. Runtime Model
4.1 Container orchestration
Docker Engine
Docker Compose v2

No orchestration layer beyond Compose.

4.2 Service execution model

Each service runs as:

isolated container
stateless (except DB-backed services)
health-checked process
4.3 Public exposure rule (STRICT)

Only:

Traefik

Everything else:

internal Docker network only
no host port exposure
5. Deployment Artifacts Model
5.1 Artifact format

Each service is delivered as:

service-name.tar

Containing:

prebuilt Docker image
immutable tag via SHA digest
5.2 Release manifest (mandatory)
release_id: 2026-06-01-r01

images:
  driver-service: sha256:abc
  admin-service: sha256:def
  clickstream-service: sha256:ghi
  gis-worker: sha256:jkl
  analytics-writer: sha256:mno
5.3 Integrity rule

Before deployment:

all images MUST match manifest
partial mismatch = deployment FAILURE
6. Directory Layout (Host)
/opt/bornemap/
  compose/
  env/
  artifacts/
    images/
    releases/
  logs/
  backups/
7. Docker Compose Architecture
7.1 Stack composition
Infrastructure layer
Traefik (Ingress)
Keycloak (Auth)
PostgreSQL (3 DBs)
RabbitMQ (Event backbone)
Application layer
Driver Service
Admin Service
Clickstream Service
GIS Worker
Analytics Writer
Frontend layer
Driver Web (served via Traefik)
Partner Dashboard
Admin Dashboard
8. Networking Model
8.1 Internal DNS convention
service.domain.internal

Examples:

postgres.platform.internal
rabbitmq.platform.internal
keycloak.auth.internal
driver.service.internal
8.2 Traffic flow
User
 ↓
Traefik
 ↓
Frontend Apps / API Gateway routes
 ↓
Backend Services
 ↓
PostgreSQL / RabbitMQ
9. Ingress Model (Traefik)
9.1 Responsibilities
TLS termination
routing
domain-based routing
load balancing (basic)
9.2 Public domains
driver.example.tn
admin.example.tn
partner.example.tn
api.example.tn
auth.example.tn
9.3 Routing rules
Domain	Target
driver.*	Driver Web API
admin.*	Admin Service
api.*	Backend APIs
auth.*	Keycloak
partner.*	Partner Dashboard
10. Database Deployment Model
10.1 Databases
DB	Purpose
keycloak_db	identity
platform_db	business + GIS
analytics_db	events
10.2 Extensions

platform_db MUST include:

PostGIS enabled
10.3 Migration rule
migrations run BEFORE service startup
migrations are NOT auto-executed at runtime
11. Service Startup Order (STRICT)
PostgreSQL
RabbitMQ
Keycloak
Traefik
Backend services
Workers
Frontend apps
12. Scaling Model
12.1 Horizontal scaling (manual)

Each service can be scaled via:

docker compose up --scale driver-service=2
12.2 Constraints
no auto-scaling
no orchestration scheduler
manual load balancing via Traefik
13. Health Check Model

Each service MUST expose:

/health

Must validate:

DB connection
dependency reachability
internal state OK
14. Observability Model
14.1 Logging
structured JSON logs
request_id required
no PII allowed
14.2 Metrics (baseline)
API latency
request count
error rate
queue depth (RabbitMQ)
GIS sync lag
15. Deployment Workflow
Step 1 — Preflight
validate host readiness
check Docker
verify env files
validate DB connectivity
validate RabbitMQ
Step 2 — Load artifacts
docker load -i driver-service.tar
docker load -i admin-service.tar
Step 3 — Validate manifest
verify image SHA
reject mismatches
Step 4 — Run migrations
platform_db
analytics_db
keycloak_db (if needed)
Step 5 — Start infrastructure
Traefik
Keycloak
Step 6 — Start services
backend services
workers
Step 7 — Start frontends
web apps via Traefik
Step 8 — Smoke tests
auth login
station fetch
GIS sync check
event ingestion check
16. Rollback Model
Levels
Level 1 — Service rollback
redeploy previous image
Level 2 — Full stack rollback
restart all services
Level 3 — DB rollback
restore from backup
Rule

Rollback MUST always be preplanned.

17. Failure Modes
DB connection failure
RabbitMQ backlog
GIS worker lag
Keycloak misconfiguration
Traefik routing failure
migration mismatch
invalid event schema
18. Backup Strategy

Before every production release:

platform_db backup
analytics_db backup
optional keycloak_db backup
19. Security Model
no exposed backend ports
only Traefik public
JWT validation required everywhere
partner isolation enforced server-side
no direct DB access from frontend
20. Summary

This deployment model enforces:

deterministic bare-metal execution
strict service isolation
event-driven backend architecture
safe multi-service orchestration via Docker Compose
predictable rollback and recovery
production-grade operational discipline without Kubernetes complexity
