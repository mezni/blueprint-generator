# Quickstart: BorneMap Phase 0

## Prerequisites

- Docker Engine 24+ with docker compose plugin
- Node.js 20+ (LTS)
- Python 3.12+
- pnpm 9+ (`npm install -g pnpm`)
- Expo Go app on iOS/Android device or simulator
- Visual Studio Code (recommended)

## First-Time Setup

```bash
# 1. Start infrastructure
docker compose up -d

# 2. Backend
cd bornemap/backend
python -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
# Copy env template
cp .env.example .env
# Run migrations
alembic upgrade head
# Start dev server
uvicorn app.main:app --reload --host 0.0.0.0 --port 8000

# 3. Web admin
cd ../web
npm install
npm run dev

# 4. Mobile (separate terminal)
cd ../mobile
pnpm install
pnpm start
```

## Verify

```bash
# Health endpoints
curl http://localhost:8000/health/live
# → {"status":"ok"}

curl http://localhost:8000/health/ready
# → {"status":"ok","database":"connected"}

# Web admin
open http://localhost:5173

# Mobile: scan QR code from Expo Go terminal
```

## Verification Checklist

- [ ] `GET /health/live` returns 200 `{"status":"ok"}`
- [ ] `GET /health/ready` returns 200 `{"status":"ok","database":"connected"}` (or 503 without PostGIS)
- [ ] Web admin loads full-screen map with CartoDB Positron tiles
- [ ] Floating panels visible (Search top-left, Stations top-right, Details bottom)
- [ ] Mobile app renders in Expo Go with Map and Favorites tabs
- [ ] Backend hot-reload: edit `bornemap/backend/app/health/router.py` → server reloads
- [ ] Web hot-reload: edit `bornemap/frontend/web/src/App.tsx` → browser reloads
- [ ] Mobile hot-reload: edit `bornemap/frontend/mobile/app/(tabs)/index.tsx` → Expo reloads

## CI Validation

```bash
# Backend
cd bornemap/backend
ruff check . && black --check . && pytest

# Frontend
cd bornemap/frontend/web
npx tsc --noEmit && npm run build

# Mobile (pnpm required)
cd bornemap/frontend/mobile
pnpm typecheck
pnpm test
```

## Branch Workflow

```bash
git checkout 003-foundation-infrastructure
# Make changes, commit, push
# Open PR against main — CI runs automatically
```
