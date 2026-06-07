# Backup & Restore

## Database Backups

### PostgreSQL Dump

```bash
# Full backup of platform_db
pg_dump -h localhost -U bornemap -d platform_db > backup_platform_$(date +%Y%m%d).sql

# Full backup of analytics_db
pg_dump -h localhost -U bornemap -d analytics_db > backup_analytics_$(date +%Y%m%d).sql

# Keycloak DB backup
pg_dump -h localhost -U keycloak -d keycloak_db > backup_keycloak_$(date +%Y%m%d).sql
```

### Restore

```bash
# Restore platform_db
psql -h localhost -U bornemap -d platform_db < backup_platform_20260101.sql
```

## Schedule

- Daily full backup
- 7-day retention on host
- Weekly backup copied to off-site storage
- Monthly archival backup
