# API Envelope Contract

**Version**: 1.0 | **Date**: 2026-06-01

All service endpoints use a standard JSON envelope.

## Success Response

HTTP 200:

```json
{
  "success": true,
  "data": { },
  "meta": { }
}
```

## Error Response

```json
{
  "success": false,
  "error": {
    "code": "STRING",
    "message": "STRING"
  }
}
```

## Standard Error Codes

| Code | Meaning |
|------|---------|
| SERVICE_UNAVAILABLE | Dependency unavailable (ready probe) |
| CONFIG_INVALID | Missing/invalid configuration |
| INTERNAL_ERROR | Unexpected failure |
