# Contract: ID Generation

## Status

**Adopted**: 2026-05-31
**Scope**: All application-layer entities across all services
**Owner**: Platform Architecture

## Format

All entity IDs follow the format:

```
{prefix}-{nanoid}
```

Where `{prefix}` is a 3-4 uppercase letter entity identifier and `{nanoid}` is a 21-character random ID using the default NanoID alphabet (A-Za-z0-9_-).

## Prefix Table

| Entity | Prefix | Char Length | Total Length | Example |
|--------|--------|-------------|--------------|---------|
| User Account | `USR-` | 4 + 21 | 25 | `USR-a1b2c3d4e5f6g7h8i9j0k` |
| Partner | `PRT-` | 4 + 21 | 25 | `PRT-x1y2z3w4v5u6t7s8r9q0p` |
| Station | `STN-` | 4 + 21 | 25 | `STN-m9n8o7p6q5r4s3t2u1v0w` |
| Charger | `CHG-` | 4 + 21 | 25 | `CHG-l0k1j2h3g4f5d6s7a8p9o` |
| Review | `REV-` | 4 + 21 | 25 | `REV-zx9cv8bn7m6l5k4j3h2g1` |

## Implementation

The ID generation function lives in the `contracts` crate (`contracts/src/lib.rs` or a dedicated `contracts/src/id.rs`):

```rust
use nanoid::nanoid;

const ID_ALPHABET: [char; 62] = [
    '0','1','2','3','4','5','6','7','8','9',
    'A','B','C','D','E','F','G','H','I','J','K','L','M',
    'N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i','j','k','l','m',
    'n','o','p','q','r','s','t','u','v','w','x','y','z',
    '-', '_',
];

pub fn generate_id(prefix: &str) -> String {
    format!("{}-{}", prefix, nanoid!(21, &ID_ALPHABET))
}
```

## Constraints

1. IDs are generated at the application layer (not by the database).
2. ID length is fixed at 25 characters total (prefix + hyphen + 21 NanoID chars).
3. All VARCHAR(26) columns in the database can accommodate the full ID.
4. IDs MUST be globally unique — collision probability for 21-char NanoID is negligible (~126 bits of entropy).
5. IDs MUST appear consistently in DB, APIs, logs, events, and UI.
6. IDs are immutable once assigned.
7. No two entity types share a prefix.

## Validation

Accept ID if and only if:
- Matches regex: `^(USR|PRT|STN|CHG|REV)-[A-Za-z0-9_-]{21}$`
- Total length = 25 characters
