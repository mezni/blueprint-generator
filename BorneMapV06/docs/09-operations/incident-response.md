# Incident Response

## Incident Levels

| Level | Description | Response Time |
|-------|-------------|---------------|
| P1 | Service down, users impacted | Immediate |
| P2 | Degraded performance | 1 hour |
| P3 | Minor issue, workaround exists | 1 business day |
| P4 | Cosmetic, non-urgent | Next release |

## Response Process

1. **Detect** — automated health check or user report
2. **Assess** — determine incident level and affected services
3. **Respond** — apply mitigation (restart service, rollback, etc.)
4. **Resolve** — confirm service restored
5. **Review** — document root cause and preventive measures

## Common Incidents

### Service Unhealthy
```bash
# Check service logs
docker logs <service-name>

# Restart service
docker compose restart <service-name>

# If container image issue, pull fresh
docker compose pull <service-name>
docker compose up -d <service-name>
```

### Database Connection Issues
```bash
# Check PostgreSQL is running
docker compose ps postgres

# Check logs
docker logs postgres

# Restart if needed
docker compose restart postgres
```
