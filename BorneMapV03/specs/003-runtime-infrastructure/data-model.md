# Data Model: Runtime Infrastructure & API Gateway

**Phase**: 1 — Design & Contracts
**Date**: 2026-05-31
**Source Spec**: [spec.md](../spec.md)

This document defines the infrastructure entities of the BorneMap runtime platform.

## Service Entities

| Entity | Type | Network | Depends On | Healthcheck |
|--------|------|---------|------------|-------------|
| `traefik` | Gateway | public, internal | — | tcp :80 |
| `postgres` | Database | internal | — | pg_isready |
| `rabbitmq` | Message broker | internal | — | rabbitmq-diagnostics status |
| `keycloak` | Identity provider | internal | postgres | /health |
| `admin-service` | Backend | internal | postgres, rabbitmq, keycloak | /health |
| `driver-service` | Backend | internal | postgres, rabbitmq, keycloak | /health |
| `clickstream-service` | Backend | internal | rabbitmq | /health |
| `gis-sync-worker` | Worker | internal | postgres, rabbitmq | /health |
| `driver-web` | Frontend | internal | — | tcp :80 |
| `admin-dashboard` | Frontend | internal | — | tcp :80 |
| `partner-dashboard` | Frontend | internal | — | tcp :80 |

## Network Entities

| Network | Driver | External | Contains |
|---------|--------|----------|----------|
| `public_network` | bridge | No | traefik |
| `internal_backend` | bridge | No | all other services |

## Route Entities

| Router | Path | Target | Middleware |
|--------|------|--------|-----------|
| `driver-api` | `/api/v1/driver/*` | driver-service:8080 | strip-prefix, rate-limit |
| `admin-api` | `/api/v1/admin/*` | admin-service:8080 | strip-prefix |
| `events-api` | `/api/v1/events/*` | clickstream-service:8080 | strip-prefix |
| `auth` | `/auth/*` | keycloak:8080 | — |
| `driver-web` | `/` | driver-web:80 | — |
| `admin-dashboard` | `/admin` | admin-dashboard:80 | — |
| `partner-dashboard` | `/partner` | partner-dashboard:80 | — |

## Environment Configuration Entities

| Variable | Scope | Purpose |
|----------|-------|---------|
| `DATABASE_URL` | postgres, backend services | Connection string |
| `POSTGRES_USER` | postgres | Auth |
| `POSTGRES_PASSWORD` | postgres | Auth |
| `POSTGRES_DB` | postgres | Database name |
| `RABBITMQ_URL` | rabbitmq, backend services | Connection string |
| `KEYCLOAK_URL` | keycloak, backend services | Auth base URL |
| `KEYCLOAK_REALM` | keycloak | Realm name |
| `KEYCLOAK_CLIENT_ID` | backend services | OAuth client |
| `TRAEFIK_DOMAIN` | traefik | DNS domain |
| `RUST_LOG` | backend services | Log level |
| `ENVIRONMENT` | all services | `local` or `production` |

## CI Pipeline Stage Entities

| Stage | Depends On | Jobs | Artifact |
|-------|-----------|------|----------|
| `lint` | — | clippy, rustfmt, eslint | — |
| `test` | lint | cargo test | — |
| `build` | lint | cargo build, npm build | compiled binaries |
| `contract-validation` | lint | DTO audit | — |
| `docker-build` | build, test, contract-validation | docker buildx per service | Docker images |
| `ghcr-publish` | docker-build | docker push | tagged images |
