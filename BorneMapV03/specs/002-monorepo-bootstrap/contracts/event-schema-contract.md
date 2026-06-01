# Event Schema Contract

**Crate**: `crates/contracts`
**File**: `src/events.rs`
**Status**: Scaffold — envelope defined, event variants enumerated

## ClickstreamEventEnvelope

```rust
struct ClickstreamEventEnvelope {
    event_id: String,        // UUID v4
    event_type: EventType,
    user_id: Option<String>,  // NanoID USR- prefix (None for anonymous events)
    session_id: String,       // UUID v4
    payload: serde_json::Value,
    timestamp: DateTime<Utc>,
    source: String,           // e.g., "driver-web", "driver-mobile"
    trace_id: String,         // UUID v4 — propagates across services
}
```

## EventType (9 v1 variants)

| Variant | Description | Source |
|---------|-------------|--------|
| `StationSearched` | User searched for stations | driver-web, driver-mobile |
| `StationViewed` | User viewed station details | driver-web, driver-mobile |
| `ChargingStarted` | User started a charging session | driver-web, driver-mobile |
| `ChargingCompleted` | Charging session ended | driver-service |
| `ReviewSubmitted` | User submitted a station review | driver-service |
| `PartnerStationCreated` | Partner created a new station | admin-service |
| `PartnerStationUpdated` | Partner updated station details | admin-service |
| `UserRegistered` | A new user registered | driver-service, admin-service |
| `ErrorOccurred` | Application error (any service) | all services |
