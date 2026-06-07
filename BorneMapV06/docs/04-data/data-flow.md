# Data Flow

## Station Data Flow

```
Partner Dashboard → Admin Service → platform_db.inventory.station
                                          │
                                          ▼
                                   Outbox Table
                                          │
                                          ▼
                                   GIS Sync Worker
                                          │
                                          ▼
                                   gis.station_location
```

## Analytics Flow

```
Frontend → Clickstream Service → RabbitMQ → Analytics Consumer
                                               │
                                               ▼
                                           analytics_db
```

## Review Flow

```
Driver App → Driver Service → platform_db.users.station_review
                                       │
                                       ▼
                                Admin moderates
                                       │
                                       ▼
                               status: approved/rejected
```
