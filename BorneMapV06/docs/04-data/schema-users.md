# Users Schema

Schema: `users` in `platform_db`

## Tables

### `user_profile`

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | Keycloak user ID |
| display_name | TEXT | Public display name |
| email | TEXT | Email address |
| created_at | TIMESTAMPTZ | Creation timestamp |

### `favorite_station`

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT (nanoid:16) | Primary key (16-char prefixed NanoID) |
| user_id | TEXT | FK to user profile |
| station_id | TEXT | FK to station |
| created_at | TIMESTAMPTZ | Creation timestamp |

*Unique constraint on (user_id, station_id)*

### `station_review`

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT (nanoid:16) | Primary key (16-char prefixed NanoID) |
| user_id | TEXT | FK to user profile |
| station_id | TEXT | FK to station |
| rating | INTEGER | 1-5 rating |
| body | TEXT | Review text |
| status | TEXT | pending / approved / rejected |
| created_at | TIMESTAMPTZ | Creation timestamp |
| updated_at | TIMESTAMPTZ | Last update timestamp |
