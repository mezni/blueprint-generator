# Driver Service — Quickstart

## API Endpoints

All endpoints are under `/api/v1/driver/*`.

### Public (no auth required, optional auth for distance)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/api/v1/driver/stations` | List visible stations (bbox/radius) |
| GET | `/api/v1/driver/stations/{id}` | Station detail with chargers + reviews |
| GET | `/api/v1/driver/stations/search` | Search stations by text |

### Authenticated (requires `registered_driver` role)

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/driver/favorites/{station_id}` | Add station to favorites |
| DELETE | `/api/v1/driver/favorites/{station_id}` | Remove station from favorites |
| GET | `/api/v1/driver/favorites` | List favorite station IDs |
| POST | `/api/v1/driver/reviews` | Create a review |
| PATCH | `/api/v1/driver/reviews/{id}` | Update own review |
| DELETE | `/api/v1/driver/reviews/{id}` | Soft-delete own review |
| GET | `/api/v1/driver/me` | Get driver profile |
| PATCH | `/api/v1/driver/me` | Update driver profile |

## Station Query Examples

### Radius query (near Tunis)
```
GET /api/v1/driver/stations?lat=36.8065&lng=10.1815&radius_km=10
```

### Search by name/city/description
```
GET /api/v1/driver/stations/search?q=Tunis
```

### Station detail
```
GET /api/v1/driver/stations/STN-01ABCDEF
```

## Auth Headers

```
Authorization: Bearer <JWT>
```

## Station Visibility Filter

Only stations matching ALL conditions are returned:
- `is_live = true`
- `deleted_at IS NULL`
- `status = 'active'`
- `is_public = true`

## Response Format

```json
{
  "success": true,
  "data": [ /* items */ ],
  "meta": {
    "page": 1,
    "size": 20,
    "total": 42,
    "total_pages": 3,
    "has_next": true,
    "has_prev": false
  }
}
```

Single-item responses use:
```json
{
  "success": true,
  "data": { /* item */ },
  "meta": {}
}
```

## Error Format

```json
{
  "success": false,
  "error": {
    "code": "NOT_FOUND",
    "message": "Station with id 'STN-XXX' not found",
    "details": null
  }
}
```

## Default Map Center (Tunisia)

- Lat: 36.8065
- Lng: 10.1815
- Default radius: 10 km
- Max radius: 50 km
