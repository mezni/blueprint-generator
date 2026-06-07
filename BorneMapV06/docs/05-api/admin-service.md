# Admin Service API

## Partner Endpoints

All partner endpoints are scoped to the authenticated partner's organization.

### `GET /stations`
List own stations.

### `POST /stations`
Create a new station.

### `PUT /stations/:id`
Update own station.

### `DELETE /stations/:id`
Soft-delete own station.

### `GET /stations/:id/chargers`
List chargers for own station.

### `POST /stations/:id/chargers`
Add a charger to own station.

### `PUT /chargers/:id`
Update own charger.

### `DELETE /chargers/:id`
Remove own charger.

### `PUT /stations/:id/availability`
Update manual availability for own station.

## Admin Endpoints

### `GET /users`
List all users.

### `GET /partners`
List all partner organizations.

### `POST /partners`
Create a partner organization.

### `PUT /partners/:id`
Update partner.

### `GET /stations`
List all stations.

### `PUT /stations/:id`
Update any station.

### `DELETE /stations/:id`
Soft-delete any station.

### `GET /stations/:id/chargers`
List chargers for any station.

### `POST /stations/:id/chargers`
Add charger to any station.

### `PUT /chargers/:id`
Update any charger.

### `DELETE /chargers/:id`
Remove any charger.

### `GET /reviews`
List all reviews (with pending status filter).

### `PUT /reviews/:id/moderate`
Approve or reject a review.

### `GET /reports`
Get aggregated platform reports.
