# Security Checklist

## Authentication & Authorization

- [ ] Only Traefik exposes public ports
- [ ] All API endpoints validate JWT (except public endpoints)
- [ ] Partner access is scoped to their own organization
- [ ] No direct database access from frontend
- [ ] Secrets stored only on host environment files

## API Security

- [ ] Rate limiting on all public endpoints
- [ ] Input validation on all request bodies
- [ ] SQL injection prevention (parameterized queries via SQLx)
- [ ] CORS configured per service

## Data Security

- [ ] No hard deletes—soft delete only for stations
- [ ] Analytics events are immutable
- [ ] Keycloak is sole identity provider
- [ ] Passwords never stored in platform database

## Infrastructure

- [ ] TLS termination at Traefik
- [ ] Docker containers run as non-root
- [ ] Database credentials rotated regularly
- [ ] Backups encrypted at rest
