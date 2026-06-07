# Analytics Schema

Database: `analytics_db`

## Rules

- Analytics lives only in `analytics_db`
- Events are immutable — never updated or deleted
- Analytics never affects system state
- Separate from business data

## Tables

### `clickstream_event`

| Column | Type | Description |
|--------|------|-------------|
| id | BIGSERIAL | Auto-incrementing PK |
| event_type | TEXT | Type of event (page_view, search, etc.) |
| payload | JSONB | Event payload data |
| user_id | TEXT | Optional authenticated user ID |
| session_id | TEXT | Browser session identifier |
| timestamp | TIMESTAMPTZ | Event timestamp |

### `aggregated_metrics`

| Column | Type | Description |
|--------|------|-------------|
| id | BIGSERIAL | Auto-incrementing PK |
| metric_name | TEXT | Metric identifier |
| period | TEXT | aggregation period (hourly/daily) |
| value | NUMERIC | Aggregated value |
| timestamp | TIMESTAMPTZ | Period timestamp |
