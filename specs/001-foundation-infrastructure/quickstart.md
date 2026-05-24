# Quickstart: BorneMap Phase 0

## Prerequisites

- Docker Engine 24+ with docker compose plugin
- Node.js 20+ (LTS)
- Python 3.12+
- Expo Go app on iOS/Android device or simulator
- Visual Studio Code (recommended)

## First-Time Setup

```bash
# 1. Start infrastructure
docker compose up -d

# 2. Backend
cd backend
python -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
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
npm install
npx expo start
```

## Verify

```bash
# Health endpoints
curl http://localhost:8000/health/live
curl http://localhost:8000/health/ready

# Web admin
open http://localhost:5173

# Mobile: scan QR code from Expo Go terminal
```

## CI Validation

```bash
# Backend
cd backend
ruff check . && black --check . && pytest

# Frontend
cd web
npx tsc --noEmit && npm run build

# Mobile
cd mobile
npx tsc --noEmit
```

## Branch Workflow

```bash
git checkout 003-foundation-infrastructure
# Make changes, commit, push
# Open PR against main — CI runs automatically
```
