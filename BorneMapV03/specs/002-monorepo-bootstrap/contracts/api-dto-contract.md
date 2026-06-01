# API DTO Contract

**Crate**: `crates/contracts`
**File**: `src/dto.rs`
**Status**: Scaffold — types defined, fields specified, to be filled per EPIC 2+

## API Versioning

All DTOs are served under `/api/v1/*`. No unversioned endpoints.
- `baseURL = "/api/v1"` in all frontend clients
- Services enforce `/api/v1/<service>/` prefix internally

## DTOs

### `StationDTO`
- `id: String` — NanoID prefix `STN-`
- `name: String`
- `partner_id: String` — NanoID prefix `PRT-`
- `address: String`
- `latitude: f64`
- `longitude: f64`
- `charger_count: u32`
- `status: StationStatus`

### `UserDTO`
- `id: String` — NanoID prefix `USR-`
- `email: String`
- `display_name: String`
- `role: Role`
- `created_at: DateTime<Utc>`

### `PartnerDTO`
- `id: String` — NanoID prefix `PRT-`
- `name: String`
- `contact_email: String`
- `status: PartnerStatus`
- `created_at: DateTime<Utc>`

### `ReviewDTO`
- `id: String` — NanoID prefix `REV-`
- `station_id: String` — NanoID prefix `STN-`
- `user_id: String` — NanoID prefix `USR-`
- `rating: u8` — 1–5
- `comment: Option<String>`
- `created_at: DateTime<Utc>`

## Enums

### `StationStatus`
- `Active`
- `Inactive`
- `Maintenance`

### `PartnerStatus`
- `Active`
- `Suspended`
- `Onboarding`
