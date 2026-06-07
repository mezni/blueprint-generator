# API Contracts — Sprint 10 Partner Dashboard

The partner dashboard consumes the existing admin-service API at `/api/v1/partner/*`.

## Envelope Format

All responses follow the standard Bornemap envelope:

### Success (list)
```json
{
  "success": true,
  "data": [...],
  "meta": { "page": 1, "size": 20, "total": 42, "total_pages": 3, "has_next": true, "has_prev": false }
}
```

### Success (single item)
```json
{
  "success": true,
  "data": { ... },
  "meta": {}
}
```

### Error
```json
{
  "success": false,
  "error": { "code": "UNAUTHENTICATED", "message": "Authentication required", "details": null }
}
```

For full entity shapes and error codes, see [data-model.md](../data-model.md).
