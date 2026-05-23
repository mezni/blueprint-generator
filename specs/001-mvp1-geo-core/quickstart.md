# Quickstart — MVP 1 Geo Core

**Branch**: `001-mvp1-geo-core` | **Audience**: implementer LLMs and human contributors.

A 10-minute walkthrough to bring up MVP 1 on a clean machine, run the smoke flows, and verify the constitutional gates pass locally. This file is also the manual test script attached to the validation report at the MVP boundary.

---

## 0. Prerequisites

- Linux or macOS (Windows via WSL2 works).
- Docker 24+ with `compose` plugin.
- Rust toolchain (rustup) — `rust-toolchain.toml` pins the version.
- Node 20+ and `pnpm` 9+.
- `sqlx-cli`: `cargo install sqlx-cli --no-default-features --features postgres,rustls`.

---

## 1. Clone & install

```bash
git clone git@github.com:mezni/Amilcar.git bornemap
cd bornemap
git checkout 001-mvp1-geo-core
pnpm -r install --frozen-lockfile
```

---

## 2. Bring up PostGIS

```bash
docker compose -f infrastructure/docker-compose.local.yml up -d bornemap-database
```

This starts `postgis/postgis:16-3.4` on `localhost:5432` with credentials `bornemap / bornemap` and database `bornemap_dev`.

Wait until ready:

```bash
docker compose -f infrastructure/docker-compose.local.yml logs -f bornemap-database \
  | grep -m1 "database system is ready to accept connections"
```

---

## 3. Apply migrations and seed

```bash
cp infrastructure/env/backend.env.example backend/.env
# Edit MOCK_ADMIN_USERNAMES=alice,bob (you choose the names)

cd backend
sqlx migrate run --source services/station-service/migrations
```

Migration `20260523_0004_seed_synthetic.sql` inserts 500 stations (all `is_test=TRUE`).

Verify:

```bash
psql "postgres://bornemap:bornemap@localhost:5432/bornemap_dev" \
  -c "SELECT COUNT(*) FROM station_domain.stations WHERE deleted_at IS NULL;"
# Expect: 500
```

---

## 4. Run the backend

```bash
# From repo root
cd backend
cargo run -p station-service
```

Smoke endpoints:

```bash
curl -s http://localhost:8000/health/live   # 200, body "OK"
curl -s http://localhost:8000/health/ready  # 200 if DB up
curl -s http://localhost:8000/metrics | head -n 5
```

---

## 5. Verify mock-login allowlist (FR-004, FR-019)

```bash
# Allowlisted user → 200 + JWT
curl -s -X POST http://localhost:8000/api/v1/auth/mock-login \
  -H 'content-type: application/json' \
  -d '{"username":"alice","role":"admin"}' | jq

# Non-allowlisted user → 403 + RFC-7807
curl -s -i -X POST http://localhost:8000/api/v1/auth/mock-login \
  -H 'content-type: application/json' \
  -d '{"username":"mallory","role":"admin"}'

# Driver role in MVP 1 → 400
curl -s -i -X POST http://localhost:8000/api/v1/auth/mock-login \
  -H 'content-type: application/json' \
  -d '{"username":"alice","role":"driver"}'
```

Decode the issued JWT at https://jwt.io (or via `jwt` CLI). Verify the claim shape:

```json
{
  "sub": "...",
  "preferred_username": "alice",
  "realm_access": { "roles": ["admin"] },
  "iat": ...,
  "exp": ...
}
```

---

## 6. Run a viewport query (FR-001, FR-007)

```bash
curl -s 'http://localhost:8000/api/v1/stations?bbox=10.0000,33.0000,11.0000,34.0000' | jq
```

Expect:

```json
{
  "viewport": { "west": 10.0, "south": 33.0, "east": 11.0, "north": 34.0 },
  "markers": [
    { "id": "...", "name": "...", "coord": [10.6, 33.5], "is_active": true, "under_maintenance": false }
  ],
  "truncated": false
}
```

Verify the **quantization** by sending sub-4-decimal precision and confirming the response viewport rounds to 4 decimals:

```bash
curl -s 'http://localhost:8000/api/v1/stations?bbox=10.000012345,33.000056789,11.0,34.0' \
  | jq .viewport
# Expect: {"west":10.0,"south":33.0001,"east":11.0,"north":34.0}
```

---

## 7. Admin CRUD smoke

```bash
TOKEN=$(curl -s -X POST http://localhost:8000/api/v1/auth/mock-login \
  -H 'content-type: application/json' \
  -d '{"username":"alice","role":"admin"}' | jq -r .access_token)

# Create
curl -s -X POST http://localhost:8000/api/v1/admin/stations \
  -H "Authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{
    "company_id": "00000000-0000-0000-0000-000000000001",
    "name": "Test Borne",
    "address": "Avenue Habib Bourguiba, Tunis",
    "coord": [10.1815, 36.8065],
    "opening_hours_osm": "Mo-Fr 08:00-20:00; Sa 09:00-13:00",
    "chargers": [{"connector": "CCS", "power_kw": 50}]
  }' | tee /tmp/created.json | jq .id

STATION_ID=$(jq -r .id /tmp/created.json)

# Read full detail
curl -s "http://localhost:8000/api/v1/stations/$STATION_ID" | jq

# Patch under_maintenance → pin should flip red on next viewport fetch
curl -s -X PATCH "http://localhost:8000/api/v1/admin/stations/$STATION_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"under_maintenance": true}' | jq

# Soft delete
curl -s -i -X DELETE "http://localhost:8000/api/v1/admin/stations/$STATION_ID" \
  -H "Authorization: Bearer $TOKEN"
# Expect: 204
```

Verify validation rejections:

```bash
# Bad connector → 422
curl -s -i -X POST http://localhost:8000/api/v1/admin/stations \
  -H "Authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"company_id":"00000000-0000-0000-0000-000000000001","name":"x","address":"y","coord":[10,36],"chargers":[{"connector":"Tesla","power_kw":50}]}'

# Bad opening_hours → 422
curl -s -i -X POST http://localhost:8000/api/v1/admin/stations \
  -H "Authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"company_id":"00000000-0000-0000-0000-000000000001","name":"x","address":"y","coord":[10,36],"opening_hours_osm":"not a valid OSM string"}'

# west > east bbox → 400
curl -s -i 'http://localhost:8000/api/v1/stations?bbox=11.0,33.0,10.0,34.0'
```

---

## 8. Bring up the admin portal

```bash
# In a new terminal
cd frontend/admin-portal
cp ../../infrastructure/env/frontend.env.example .env
# Ensure VITE_API_BASE_URL=http://localhost:8000
pnpm dev
```

Open http://localhost:5173.

Verify by hand:

1. Log in with the allowlisted username (`alice`). Token persists across reloads.
2. Navigate to **/admin/stations**. Table renders the 500 seeded stations + the one you just created.
3. Edit a station; toggle `under_maintenance`; save; confirm the row updates.
4. Open `/admin/stations/new`; submit a valid station; confirm 201 + table refresh.
5. Open the map view (`/map`). Pan to Tunis; confirm pins render and clustering activates in dense areas. Repeat pan/zoom; observe DevTools Network → only `moveend`/`zoomend` triggers requests, and the cache returns identical viewports without refetching.
6. Click a pin → detail panel opens → **Navigate** button opens a new tab with `https://www.google.com/maps/dir/?api=1&destination={lat},{lng}`.

---

## 9. Bring up the mobile app

```bash
cd frontend/mobile-app
pnpm install
pnpm exec expo start --tunnel
```

Scan the QR code with **Expo Go** on a device that meets the floor (Android 10+ / iOS 15+).

Verify:

1. Launching on an OS below the floor shows the **"Update your OS"** screen and refuses to navigate further (FR-020).
2. On a supported OS, the map centers on Tunisia.
3. Pan rapidly — `tracksViewChanges={false}` keeps frame rate stable.
4. Tap a pin → bottom sheet opens with detail.
5. Tap **Navigate** → OS hands off to the installed map app (or browser tab).

---

## 10. Run the test suite (constitutional gate VI + V)

```bash
# From repo root
cd backend
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
SQLX_OFFLINE=true cargo build --workspace
cargo test --workspace            # Boots PostGIS via Testcontainers

cd ../frontend
pnpm -r lint
pnpm -r typecheck
pnpm -r test
```

All commands must exit 0.

---

## 11. Verify OpenAPI drift gate

```bash
cd backend
cargo run -p openapi-spec --release > /tmp/openapi.runtime.json
diff /tmp/openapi.runtime.json frontend/packages/api-client/openapi.json
# No diff = pass
```

---

## 12. Validation cohort handoff

When the smoke flows above pass:

1. Deploy the staging environment.
2. Recruit the cohort per `docs/plan.md` §9.1 (≥ 5 drivers + ≥ 2 admins; include at least one Galaxy A33 5G + iPhone 11 + each compatibility floor).
3. Use this quickstart as the scripted-task source for the sessions.
4. Author `specs/001-mvp1-geo-core/validation.md` with the per-task success/failure breakdown and SUS score.
5. The platform owner writes `specs/001-mvp1-geo-core/decision.md` to proceed/adjust/kill before any MVP 2 work begins.

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| Server refuses to start: "MOCK_ADMIN_USERNAMES must be set" | Missing env var (FR-019, fail-closed) | Set the env var in `backend/.env`. |
| `cargo test` fails: "Docker not available" | Testcontainers needs Docker socket | Start Docker; ensure user is in the `docker` group. |
| Mobile app crashes at launch on old OS | Below floor (FR-020) | Update OS or test on a supported device. |
| OpenAPI drift gate fails | Hand-edited `openapi.json` | Regenerate via `cargo run -p openapi-spec`. Do not edit the snapshot manually. |
| `ST_DWithin` is slow | GiST index missing | Verify with `\d+ station_domain.stations` in psql; check `stations_location_gix`. |
