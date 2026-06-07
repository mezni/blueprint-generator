Environment & Configuration Specification (v3.0)
1. Purpose

This document defines the complete runtime configuration model for the Bornemap platform.

It governs:

Environment separation (dev/prod/local)
Service configuration ownership
Secrets management rules
Infrastructure connectivity (PostgreSQL, RabbitMQ, Keycloak, Traefik)
Configuration validation and startup safety
Cross-service configuration boundaries

It is the single source of truth for runtime configuration behavior.

2. Core Principles
2.1 Host-controlled configuration

All configuration is injected via:

.env files on host
Docker Compose environment injection

Rules:

❌ No secrets in Git
❌ No hardcoded environment-specific values
✅ Only .env.example is committed
2.2 Service isolation rule (STRICT)

Each service:

owns its own .env
MUST NOT import another service's env
MUST NOT rely on shared runtime env files
2.3 No hidden coupling rule

Forbidden:

cross-service env dependencies
implicit global config
shared mutable runtime config
2.4 Fail-fast configuration

All services MUST:

crash on missing required env vars
reject invalid enum values
validate URLs / ports on startup
log resolved config (redacted secrets)
2.5 Registry-agnostic deployment

The system MUST NOT assume:

Docker Hub
GHCR
any cloud registry

Images are injected at runtime only.

2.6 Layer precedence (highest wins)
Code defaults
.env.example defaults
environment-specific .env
host runtime injection
manual override (emergency only)
3. Environment Model
3.1 Supported environments
Environment	Purpose
local	developer machine
dev	shared development
prod	production
3.2 Environment variable
APP_ENV=local | dev | prod

Rules:

prod disables debug features
dev enables verbose logging
local allows relaxed configuration checks
4. Service Configuration Ownership
Service	Config File
Driver Service	driver-service.env
Admin Service	admin-service.env
Clickstream Service	clickstream.env
GIS Worker	gis-worker.env
Analytics Writer	analytics.env
Keycloak	keycloak.env
Traefik	traefik.env
5. Global Naming Conventions
5.1 Prefix system
Prefix	Purpose
APP_*	application metadata
LOG_*	logging
AUTH_*	authentication
DB_*	database
RABBITMQ_*	messaging
GIS_*	spatial system
CLICKSTREAM_*	event ingestion
ANALYTICS_*	analytics system
5.2 Required validation rule

Every variable MUST define:

required / optional
default value (if optional)
service ownership
6. Core Infrastructure Configuration
6.1 Traefik (Ingress Layer)

Only public entrypoint.

TRAEFIK_HTTP_PORT=80
TRAEFIK_HTTPS_PORT=443
TRAEFIK_TLS_ENABLED=true

Domains:

TRAEFIK_DOMAIN_DRIVER=driver.example.tn
TRAEFIK_DOMAIN_PARTNER=partner.example.tn
TRAEFIK_DOMAIN_ADMIN=admin.example.tn
TRAEFIK_DOMAIN_API=api.example.tn
TRAEFIK_DOMAIN_AUTH=auth.example.tn
6.2 Keycloak (Identity Provider)
KEYCLOAK_HTTP_PORT=8080
KEYCLOAK_REALM=bornemap
KEYCLOAK_PUBLIC_URL=https://auth.example.tn

Admin bootstrap:

KEYCLOAK_ADMIN_USER=admin
KEYCLOAK_ADMIN_PASSWORD=change-me

Database:

KEYCLOAK_DB_HOST=postgres.internal
KEYCLOAK_DB_NAME=keycloak_db
KEYCLOAK_DB_USER=keycloak_user
KEYCLOAK_DB_PASSWORD=change-me
6.3 Platform Database (PostgreSQL)
PLATFORM_DB_HOST=postgres.internal
PLATFORM_DB_NAME=platform_db
PLATFORM_DB_USER=platform_user
PLATFORM_DB_PASSWORD=change-me
PLATFORM_DB_SSL_MODE=disable
PLATFORM_DB_MAX_CONNECTIONS=20

Used by:

Driver Service
Admin Service
GIS Worker
6.4 Analytics Database
ANALYTICS_DB_HOST=postgres.internal
ANALYTICS_DB_NAME=analytics_db
ANALYTICS_DB_USER=analytics_user
ANALYTICS_DB_PASSWORD=change-me

Used by:

Analytics Writer
Clickstream Service
6.5 RabbitMQ (Event Backbone)
RABBITMQ_HOST=rabbitmq.internal
RABBITMQ_PORT=5672
RABBITMQ_USER=analytics
RABBITMQ_PASSWORD=change-me
RABBITMQ_VHOST=/bornemap

Queues:

RABBITMQ_EXCHANGE_CLICKSTREAM=clickstream.topic
RABBITMQ_QUEUE_CLICKSTREAM_RAW=clickstream.raw
RABBITMQ_QUEUE_CLICKSTREAM_DLQ=clickstream.dlq
7. Service-Level Configuration
7.1 Driver Service
DRIVER_SERVICE_PORT=8081
APP_ENV=prod

Auth:

AUTH_ISSUER=https://auth.example.tn/realms/bornemap
AUTH_JWKS_URL=https://auth.example.tn/realms/bornemap/protocol/openid-connect/certs
AUTH_AUDIENCE=bornemap-api

Map defaults:

MAP_DEFAULT_LAT=36.8065
MAP_DEFAULT_LNG=10.1815
MAP_DEFAULT_RADIUS_KM=10
MAP_MAX_RADIUS_KM=50
7.2 Admin Service
ADMIN_SERVICE_PORT=8082

Rules:

PARTNER_DELETE_BLOCK_ACTIVE_STATIONS=true
REPORTING_DEFAULT_WINDOW_DAYS=30
7.3 GIS Worker
GIS_WORKER_POLL_INTERVAL_MS=5000
GIS_WORKER_BATCH_SIZE=50
GIS_DEFAULT_SRID=4326

Behavior:

consumes outbox events
syncs station geometry
idempotent execution required
7.4 Clickstream Service
CLICKSTREAM_PORT=8083
CLICKSTREAM_BATCH_SIZE=100
CLICKSTREAM_ACCEPT_ANONYMOUS=true
CLICKSTREAM_ENFORCE_EVENT_ID=true
7.5 Analytics Writer
ANALYTICS_BATCH_SIZE=200
ANALYTICS_FLUSH_INTERVAL_MS=2000
ANALYTICS_RETENTION_DAYS=90
8. Frontend Configuration
8.1 Web Apps (Vite)
VITE_API_BASE_URL=https://api.example.tn
VITE_AUTH_BASE_URL=https://auth.example.tn
VITE_REALM=bornemap
VITE_SUPPORTED_LANGUAGES=ar,fr

Map:

VITE_MAP_LAT=36.8065
VITE_MAP_LNG=10.1815
8.2 Mobile App (Expo)
EXPO_PUBLIC_API_BASE_URL=
EXPO_PUBLIC_AUTH_BASE_URL=
EXPO_PUBLIC_REALM=bornemap
EXPO_PUBLIC_LANGUAGES=ar,fr
9. Security & Secrets Model
9.1 Secret classification
Level	Type
L1	config
L2	credentials
L3	bootstrap/admin secrets
9.2 Rules
secrets NEVER in Git
secrets injected at runtime
secrets rotated in production
.env.example only safe defaults
10. Configuration Validation Rules

All services MUST:

On startup
validate required envs
validate enums
validate URLs
validate ports
log sanitized config
Failure behavior

If invalid config:

➡️ service MUST crash immediately

(no fallback mode in production)

11. Service Discovery Convention

Internal DNS:

<service>.<domain>.internal

Examples:

postgres.platform.internal
rabbitmq.platform.internal
keycloak.auth.internal
12. Observability Configuration (NEW)
LOG_LEVEL=info
LOG_FORMAT=json
REQUEST_ID_HEADER=x-request-id

Rules:

structured logs required
request correlation mandatory
no PII in logs
13. Feature Flags (Controlled Use Only)
FF_ENABLE_REVIEWS=true
FF_ENABLE_GIS_SYNC=true
FF_ENABLE_ANALYTICS=true

Rules:

must be explicit
no hidden flags
no dynamic remote config in MVP
14. Operational Rules
14.1 Startup sequence dependency
Postgres
RabbitMQ
Keycloak
Backend services
Workers
Frontends
14.2 Config observability

At startup:

log resolved config
redact secrets
include APP_ENV + APP_NAME
15. Summary

This configuration system enforces:

strict service isolation
deterministic startup behavior
no hidden coupling
fully container-agnostic deployment
safe multi-environment operations
production-grade failure handling
