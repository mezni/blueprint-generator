# GDPR Compliance: Authentication Data

## Account Deletion (FR-014)

Users can request account deletion through the following flow:

1. User submits deletion request via the account settings page
2. Identity provider marks the account as `deleted` (soft delete)
3. After 30-day grace period, personal data is permanently removed
4. User cannot re-authenticate once deletion is requested

### Implementation

- Account status transitions to `deleted` in Keycloak
- `deleted_at` timestamp records when deletion was requested
- Automatic purge of soft-deleted accounts runs via cron (30-day window)
- Auth audit events track the deletion lifecycle
- Users can cancel deletion during the 30-day grace period

## Data Export (FR-015)

Users can export their personal data in machine-readable format (JSON):

- Profile information: email, display name, role
- Account history: created at, last login, account status
- Auth activity: login events, logout events, token refreshes (last 12 months)
- Does NOT include: passwords, security credentials, internal system data

### Export Format

```json
{
  "export_date": "2026-06-01T00:00:00Z",
  "user": {
    "id": "USR-abc123",
    "email": "user@example.com",
    "display_name": "John Doe",
    "role": "registered_driver",
    "created_at": "2026-01-15T10:30:00Z",
    "status": "active"
  },
  "auth_events": [
    {
      "event_type": "login_success",
      "timestamp": "2026-05-31T12:00:00Z",
      "client_id": "driver-web"
    }
  ]
}
```
