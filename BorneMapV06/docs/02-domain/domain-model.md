# Domain Model

## Core Entities

### Station
A physical EV charging location with one or more chargers.

- Lives in `inventory.station`
- Has a geographic position (lat/lng)
- Has address details and contact info
- Can be active or inactive (soft delete)
- Owned by a partner organization

### Charger
An individual charging unit at a station.

- Lives in `inventory.charger`
- Belongs to exactly one station
- Has connector type, power rating, status

### Station Availability
Manual availability indicator for a station.

- Lives in `inventory.station_availability`
- Updated by partners
- Reflects current operational status

### Review
User-submitted rating and feedback for a station.

- Lives in `users.station_review`
- Belongs to one registered driver and one station
- Includes rating score and text

### Favorite
A registered driver's bookmark for a station.

- Lives in `users.favorite_station`
- Belongs to one registered driver and one station

### Clickstream Event
An immutably recorded user action for analytics.

- Lives in `analytics_db`
- Never affects system state
- Used for aggregation and reporting
