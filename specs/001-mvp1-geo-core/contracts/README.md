# Contracts — MVP 1 Geo Core

This directory holds the API contract for MVP 1.

## File

- `openapi.yaml` — OpenAPI 3.1 specification.

## Authority

The **runtime source of truth** for the OpenAPI contract is the Rust backend — `utoipa` annotations on handlers and DTOs. A build-time bin (`backend/libs/openapi-spec`) prints the generated spec to `openapi.json`.

This `openapi.yaml` file is the **design-time pre-image**:

- It is hand-authored at planning time so contract tests can be scaffolded before code exists (constitution Principle V — Test-First).
- Once the backend handlers exist, CI generates `openapi.json` from `utoipa` and runs `oasdiff` against this file to verify they remain semantically equivalent.
- Any divergence is a contract violation: fix the handlers, do **not** edit this file to match.

## How To Use

1. **Implementer LLM**: read this file to scaffold:
   - DTO structs in Rust (`station-service::station::models`) annotated with `utoipa::ToSchema`.
   - Handler stubs in Rust annotated with `utoipa::path`.
   - Contract tests in `backend/services/station-service/tests/*.rs` that assert request/response shapes match.
2. **CI**: after backend builds, regenerate `openapi.json` and diff against this file (semantic, not byte-equal). Drift = build failure.
3. **Frontend**: regenerate `@bornemap/api-client` from the runtime `openapi.json` (not this file) — there is one source of truth at runtime, and it is the Rust handler set.

## Endpoint Summary

| Method | Path | Purpose | Auth |
|---|---|---|---|
| `POST` | `/api/v1/auth/mock-login` | Issue mock JWT | none |
| `GET` | `/api/v1/stations` | List stations in a viewport | none |
| `GET` | `/api/v1/stations/{id}` | Full station detail | none |
| `POST` | `/api/v1/admin/stations` | Create station | admin |
| `PATCH` | `/api/v1/admin/stations/{id}` | Partial update | admin |
| `DELETE` | `/api/v1/admin/stations/{id}` | Soft delete | admin |
| `GET` | `/api/v1/admin/stations` | Admin list with `include_deleted` / `include_test` filters | admin |
| `GET` | `/health/live` | Liveness | none |
| `GET` | `/health/ready` | Readiness (DB check) | none |
| `GET` | `/metrics` | Prometheus exposition | none (private network) |

## Error Format

All non-2xx responses follow **RFC 7807 (Problem Details for HTTP APIs)** with `Content-Type: application/problem+json`. See the `Problem` schema in `openapi.yaml`.
