# Driver Service API

## Public Endpoints

### `GET /stations`
List active stations with pagination and filtering.

**Query parameters:** `lat`, `lng`, `radius`, `connector_type`, `page`, `per_page`

### `GET /stations/:id`
Get station details including chargers.

### `GET /stations/nearby`
Find stations near a location.

**Query parameters:** `lat`, `lng`, `radius` (km)

### `GET /stations/markers`
Get station markers for map display (lightweight, clustered).

**Query parameters:** `ne_lat`, `ne_lng`, `sw_lat`, `sw_lng`, `zoom`

### `GET /stations/search`
Full-text search across stations.

**Query parameters:** `q`, `filters`, `page`, `per_page`

### `GET /stations/:id/reviews`
Get public reviews for a station.

## Authenticated Endpoints (Registered Driver)

### `GET /favorites`
List user's favorite stations.

### `POST /favorites`
Add station to favorites.

### `DELETE /favorites/:station_id`
Remove station from favorites.

### `POST /stations/:id/reviews`
Create a review.

### `PUT /reviews/:id`
Update own review.

### `DELETE /reviews/:id`
Delete own review.

### `GET /profile`
Get user profile.

### `PUT /profile`
Update user profile.
