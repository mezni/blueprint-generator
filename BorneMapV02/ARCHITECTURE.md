# BorneMap Architecture

**Branch**: `004-mobile-canvas`

## Repository Tree

```
borne-map/
├── .github/
│   └── workflows/
│       └── ci.yml
├── AGENTS.md
├── ARCHITECTURE.md           # This file
├── README.md
├── apps/
│   └── mobile-driver/         # Cross-Platform Client (iOS / Android / Web)
│       ├── App.js
│       ├── package.json
│       └── src/
│           ├── components/
│           │   └── StationCard.js
│           ├── screens/
│           │   └── MapScreen.js
│           └── services/
│               └── api.js
├── backend/                   # Unified Rust Core Workspace
│   ├── Cargo.toml
│   ├── api-service/
│   │   └── src/
│   │       ├── main.rs
│   │       └── domains/
│   │           └── locate/
│   │               ├── mod.rs
│   │               ├── model.rs
│   │               └── routes.rs
│   ├── core/                    # Shared library (empty crate)
│   ├── db/
│   │   ├── migrations/
│   │   │   └── 20260528000000_init_spatial_schema.sql
│   │   └── seeds/
│   │       └── demo_data.sql
│   └── infra/                   # Database pool utility crate
├── deployments/
│   └── docker-compose.yml
└── specs/
    ├── 001-mobile-driver-scaffold/
    ├── 002-backend-integration/
    ├── 003-database-persistence/
    └── 004-mobile-canvas/
        ├── spec.md
        ├── plan.md
        ├── research.md
        ├── data-model.md
        ├── quickstart.md
        ├── contracts/
        └── tasks.md
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust (Actix-web 4.4) |
| Database | PostgreSQL 15 + PostGIS 3.3 |
| Mobile Client | React Native (Expo SDK 51) |
| Runtime | Node.js v24.16.0 / npm v11.13.0 |

## Data Contracts

| Entity | ID Pattern | Example |
|--------|-----------|---------|
| Partner | `^prt-[a-f0-9]{8}$` | `prt-a1b2c3d4` |
| Station | `^stn-[a-f0-9]{8}$` | `stn-e3b0c442` |
| Charger | `^chg-[a-f0-9]{8}$` | `chg-7b2a19f4` |

## Quickstart

See [specs/004-mobile-canvas/quickstart.md](./specs/004-mobile-canvas/quickstart.md).
