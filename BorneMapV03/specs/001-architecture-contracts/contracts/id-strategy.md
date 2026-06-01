# ID Strategy

## Purpose

Define the identifier format and prefix conventions for all BorneMap entities.
All IDs MUST be NanoID with type-specific prefixes.

## Version

1.0.0

## Prefix Assignment

| Prefix | Entity | Example |
|--------|--------|---------|
| `USR-` | User account | `USR-4f3aBc9kQ2mX` |
| `PRT-` | Partner organization | `PRT-8kLmN1pQr5vW` |
| `STN-` | Charging station | `STN-2xYzR7tU9eP` |
| `CHG-` | Charger unit | `CHG-6hJkL3mN8oP` |
| `REV-` | Station review | `REV-1aBcD4eF7gH` |

## Format Rules

- **Algorithm**: NanoID (URL-safe alphabet, 21-character random suffix)
- **Case-sensitive**: Prefix uppercase, suffix mixed-case
- **Collation**: `C` (case-sensitive) in PostgreSQL
- **Storage**: `TEXT` or `VARCHAR(24)` columns

## Consistency Rule

The same ID format MUST be used in: database (primary keys, foreign keys),
REST API responses and requests, structured logs, events, and UI displays.

## Enforcement

All migration files MUST use the prefix generators. API responses MUST include
prefixed IDs. No auto-increment integers or UUIDv4 as primary keys.
