# System Architecture

## High-Level Diagram

```
┌────────────────────────────────────────────────────────────┐
│                        Traefik                             │
│                    (Public Entrypoint)                      │
└─────┬──────────┬──────────┬──────────┬─────────────────────┘
      │          │          │          │
┌─────▼──┐ ┌─────▼──┐ ┌─────▼──┐ ┌─────▼────────┐
│ Driver │ │ Admin  │ │Click-  │ │ Partner     │
│ Service│ │ Service│ │stream  │ │ Dashboard   │
│(Rust)  │ │(Rust)  │ │Service │ │(React/Vite) │
└────┬───┘ └────┬───┘ │(Rust)  │ └──────────────┘
     │          │     └───┬────┘
     │          │         │
┌────▼──────────▼─────────▼──────────────────────────────┐
│                   PostgreSQL                             │
│  ┌─────────────┐  ┌─────────────┐  ┌────────────────┐  │
│  │ platform_db  │  │ analytics_db│  │  keycloak_db   │  │
│  │ inventory    │  │ events      │  │  (Keycloak)    │  │
│  │ users        │  │ aggregates  │  │                │  │
│  │ gis          │  │             │  │                │  │
│  └─────────────┘  └─────────────┘  └────────────────┘  │
└─────────────────────────────────────────────────────────┘
                         │
                    ┌────▼────┐
                    │  GIS    │
                    │  Sync   │
                    │  Worker │
                    │ (Rust)  │
                    └─────────┘
```

## Key Design Decisions

- **Identity** is managed by Keycloak (external)
- **Business data** lives in `platform_db` (inventory, users, gis schemas)
- **Analytics** lives in `analytics_db` (separate database)
- **GIS** is a derived layer, not a source of truth
- **Events** flow through RabbitMQ to analytics
- **All public entry** goes through Traefik
