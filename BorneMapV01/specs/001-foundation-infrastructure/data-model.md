# Data Model: Phase 0

## Persistent Entities

### users

Admin user accounts for admin portal access.

| Column | Type | Constraints |
|--------|------|-------------|
| id | SERIAL | PRIMARY KEY |
| username | VARCHAR(100) | NOT NULL, UNIQUE |
| email | VARCHAR(255) | NOT NULL, UNIQUE |
| role | VARCHAR(50) | NOT NULL, DEFAULT 'viewer' |
| password_hash | VARCHAR(255) | NULLABLE (set on first login) |
| is_active | BOOLEAN | NOT NULL, DEFAULT true |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |

**SQL (PostgreSQL):**

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    role VARCHAR(50) NOT NULL DEFAULT 'viewer',
    password_hash VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### partners

Partner organizations that own or operate charging stations.

| Column | Type | Constraints |
|--------|------|-------------|
| id | SERIAL | PRIMARY KEY |
| name | VARCHAR(255) | NOT NULL |
| contact_email | VARCHAR(255) | NULLABLE |
| phone | VARCHAR(50) | NULLABLE |
| is_active | BOOLEAN | NOT NULL, DEFAULT true |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |

**SQL:**

```sql
CREATE TABLE partners (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    contact_email VARCHAR(255),
    phone VARCHAR(50),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### stations

EV charging station locations owned/operated by partners.

| Column | Type | Constraints |
|--------|------|-------------|
| id | SERIAL | PRIMARY KEY |
| partner_id | INTEGER | FK → partners(id), NULLABLE |
| name | VARCHAR(255) | NOT NULL |
| operator | VARCHAR(255) | NULLABLE |
| address | VARCHAR(500) | NULLABLE |
| location | GEOGRAPHY(Point, 4326) | NULLABLE, GiST index |
| plug_types | TEXT[] | NULLABLE |
| speed_kw | FLOAT | NULLABLE |
| is_active | BOOLEAN | NOT NULL, DEFAULT true |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |

**SQL:**

```sql
CREATE TABLE stations (
    id SERIAL PRIMARY KEY,
    partner_id INTEGER REFERENCES partners(id),
    name VARCHAR(255) NOT NULL,
    operator VARCHAR(255),
    address VARCHAR(500),
    location GEOGRAPHY(Point, 4326),
    plug_types TEXT[],
    speed_kw FLOAT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_stations_location ON stations USING GIST (location);
```

### chargers

Individual charger connectors at a station.

| Column | Type | Constraints |
|--------|------|-------------|
| id | SERIAL | PRIMARY KEY |
| station_id | INTEGER | FK → stations(id), NULLABLE |
| connector | VARCHAR(50) | NOT NULL (CCS, Type2, CHAdeMO, GBT) |
| power_kw | NUMERIC | NULLABLE |
| status | VARCHAR(50) | NOT NULL, DEFAULT 'available' |
| is_active | BOOLEAN | NOT NULL, DEFAULT true |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() |

**SQL:**

```sql
CREATE TABLE chargers (
    id SERIAL PRIMARY KEY,
    station_id INTEGER REFERENCES stations(id),
    connector VARCHAR(50) NOT NULL,
    power_kw NUMERIC,
    status VARCHAR(50) NOT NULL DEFAULT 'available',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Alembic Migrations

- `001_create_stations.py` — empty revision that establishes Alembic baseline.
- `002_create_users.py` — creates the `users` table.
- `003_create_partners.py` — creates the `partners` table.
- `004_create_stations.py` — creates the `stations` table with spatial column.
- `005_create_chargers.py` — creates the `chargers` table.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /api/v1/users | List all users |
| POST | /api/v1/users | Create a new user |
| GET | /api/v1/users/{id} | Get user by ID |
| PATCH | /api/v1/users/{id} | Update user (including is_active toggle) |
| DELETE | /api/v1/users/{id} | Delete a user |
| GET | /api/v1/partners | List all partners |
| POST | /api/v1/partners | Create a partner |
| PATCH | /api/v1/partners/{id} | Update partner |
| PATCH | /api/v1/partners/{id}/toggle-active | Toggle partner active status |
| DELETE | /api/v1/partners/{id} | Delete a partner |
| GET | /api/v1/stations | List all stations |
| POST | /api/v1/stations | Create a station |
| PATCH | /api/v1/stations/{id} | Update station |
| PATCH | /api/v1/stations/{id}/toggle-active | Toggle station active status |
| DELETE | /api/v1/stations/{id} | Delete a station |
| GET | /api/v1/chargers | List all chargers |
| POST | /api/v1/chargers | Create a charger |
| PATCH | /api/v1/chargers/{id} | Update charger |
| PATCH | /api/v1/chargers/{id}/toggle-active | Toggle charger active status |
| DELETE | /api/v1/chargers/{id} | Delete a charger |

## Notes

- Database health is verified by `/health/ready` which runs
  `SELECT 1` against PostGIS
- Connection string: `postgresql+asyncpg://borne:map@localhost:5432/borne`
- Schema: `public` (default)
