# Deployment Runbook

## Principles

- Fully manual deployment — no automated production deploy from CI
- GitHub Actions used only for: build, test, lint, Docker image build
- Docker images are built by CI, manually deployed to production

## Deployment Steps

### 1. Prepare Host

```bash
# Install Docker and Docker Compose on bare metal host
ssh <host>
apt install docker.io docker-compose-v2
```

### 2. Copy Environment Files

```bash
scp infra/env/.env.prod <host>:/opt/bornemap/.env
scp infra/env/driver-service.env.prod <host>:/opt/bornemap/
# ... repeat for each service
```

### 3. Deploy

```bash
docker compose -f infra/compose/docker-compose.yml \
               -f infra/compose/docker-compose.prod.yml \
               up -d
```

### 4. Apply Migrations

```bash
# Run migration scripts in order from db/migrations/
```

### 5. Verify

- Check all services are healthy
- Verify public endpoints respond
- Confirm Keycloak realm imported
- Run smoke tests
