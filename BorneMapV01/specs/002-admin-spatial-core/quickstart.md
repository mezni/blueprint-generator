# Phase 1 Quickstart — Admin Spatial Core

## Prerequisites

- Phase 0 foundation running (Docker Compose, backend, database)
- All Alembic migrations applied (001–005)

## Setup

```bash
# Start the stack
docker compose up -d

# Apply migrations
cd bornemap/backend
alembic upgrade head

# Start frontend
cd ../frontend/web
pnpm dev
```

## Verification Checklist

1. **Map loads**: Open `http://localhost:5173` → Full-screen map with CartoDB Positron tiles
2. **Station markers**: Existing stations appear as custom green markers
3. **Click to add**: Click map → form opens → fill details → save → marker appears
4. **Nearby search**: Use search field → map centers on location → nearby stations listed
5. **Auth required**: Without `X-Mock-Token`, POST/PATCH/DELETE return 401
6. **Viewport filter**: Pan/zoom → station list updates to show only visible stations
7. **Data table**: Toggle to table view → stations listed with search and sort

## Test Users

```json
{"sub":"1","username":"admin","roles":["admin"],"iat":0,"exp":9999999999}
```

Encode to base64 and pass as `X-Mock-Token` header.

## Spatial Query Test

```bash
# Nearby stations (Tunis center, 20 km radius)
curl "http://localhost:8000/api/v1/stations/nearby?lat=36.8065&lng=10.1815&radius_m=20000"

# Create a station
curl -X POST "http://localhost:8000/api/v1/stations" \
  -H "Content-Type: application/json" \
  -H "X-Mock-Token: eyJzdWIiOiIxIiwidXNlcm5hbWUiOiJhZG1pbiIsInJvbGVzIjpbImFkbWluIl0sImlhdCI6MCwiZXhwIjo5OTk5OTk5OTk5fQ==" \
  -d '{"name":"Test Station","location":{"coordinates":[10.1815,36.8065],"type":"Point"},"plug_types":["CCS","Type2"],"speed_kw":150}'
```
