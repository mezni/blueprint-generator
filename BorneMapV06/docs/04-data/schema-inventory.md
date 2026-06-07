# Inventory Schema

Schema: `inventory` in `platform_db`

## Tables

### `station`

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT (nanoid:16) | Primary key (16-char prefixed NanoID) |
| partner_id | TEXT | Owner partner organization |
| name | TEXT | Station name |
| address | TEXT | Physical address |
| latitude | DOUBLE PRECISION | GPS latitude |
| longitude | DOUBLE PRECISION | GPS longitude |
| status | TEXT | active / inactive (soft delete) |
| created_at | TIMESTAMPTZ | Creation timestamp |
| updated_at | TIMESTAMPTZ | Last update timestamp |

### `charger`

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT (nanoid:16) | Primary key (16-char prefixed NanoID) |
| station_id | TEXT | FK to station |
| connector_type | TEXT | Type of connector (Type 2, CCS, CHAdeMO) |
| power_kw | NUMERIC | Power rating in kW |
| status | TEXT | available / occupied / offline |
| created_at | TIMESTAMPTZ | Creation timestamp |

### `station_availability`

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT (nanoid:16) | Primary key (16-char prefixed NanoID) |
| station_id | TEXT | FK to station |
| is_available | BOOLEAN | Manual availability flag |
| updated_by | TEXT | Partner user who updated |
| updated_at | TIMESTAMPTZ | Last update timestamp |
