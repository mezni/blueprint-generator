# Event Taxonomy

## Page View Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `page_view` | `{ page, referrer }` | Navigation to any page |
| `station_view` | `{ station_id }` | View station detail |
| `search_executed` | `{ query, filters, results_count }` | Search submission |
| `filter_applied` | `{ filter_type, filter_value }` | Filter change |

## Interaction Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `map_pan` | `{ center_lat, center_lng, zoom }` | Map pan/zoom |
| `marker_click` | `{ station_id }` | Click station marker |
| `favorite_add` | `{ station_id }` | Add favorite |
| `favorite_remove` | `{ station_id }` | Remove favorite |
| `review_submitted` | `{ station_id, rating }` | Submit review |
