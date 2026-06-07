# Technology Decisions

## Backend

| Technology | Decision | Rationale |
|------------|----------|-----------|
| Rust | Backend services | Performance, type safety, reliability for API services |
| Actix-Web | Web framework | High-performance, actor-based Rust web framework |
| SQLx | Database driver | Async PostgreSQL driver with compile-time query checking |
| PostgreSQL | Database | PostGIS support, reliability, single-database simplicity |
| RabbitMQ | Message broker | Event streaming for analytics pipeline |
| Keycloak | Identity | Mature, standards-compliant identity provider |

## Frontend

| Technology | Decision | Rationale |
|------------|----------|-----------|
| React + Vite | Web apps | Fast dev experience, modern React patterns |
| React Native Expo | Mobile app | Cross-platform mobile from single codebase |
| Tailwind CSS | Styling | Utility-first, rapid UI development |
| shadcn/ui | Component base | Accessible, customizable component library |
| React Query | Data fetching | Server state management, caching |
| React Router | Routing | Standard React routing solution |
| Leaflet | Maps | Lightweight open-source map library |

## Infrastructure

| Technology | Decision | Rationale |
|------------|----------|-----------|
| Docker Compose | Orchestration | Simple deployment on bare metal |
| Traefik | Reverse proxy | Automatic TLS, Docker integration |
