# API Overview

## Base URLs

| Service | Base URL |
|---------|----------|
| Driver Service | `/api/driver/v1` |
| Admin Service | `/api/admin/v1` |
| Clickstream Service | `/api/clickstream/v1` |

## Authentication

- Public endpoints — no auth required
- Authenticated endpoints — Bearer JWT token in `Authorization` header
- JWT issued by Keycloak
- Token validated by each service

## Error Format

```json
{
  "error": {
    "code": "STATION_NOT_FOUND",
    "message": "Station with id 'xxx' not found"
  }
}
```

## Pagination

List endpoints accept `page` and `per_page` query parameters.
Responses include `total`, `page`, `per_page`, and `total_pages`.
