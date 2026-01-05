# System Architecture - Expense Tracker

A comprehensive guide to the multi-environment Docker architecture for the Expense Tracker application.

**Table of Contents**: [Overview](#overview) | [Architecture](#architecture) | [Networking](#networking) | [Security](#security) | [Deployment](#deployment)

---

## Overview

### What is This System?

The Expense Tracker is a **family budget management application** with:
- ✅ Multi-user expense tracking
- ✅ Budget management and alerts
- ✅ Credit card tracking
- ✅ Advanced analytics and reports
- ✅ CLI and Web API interfaces

### Key Innovation: Dual-Environment Setup

Both **development** and **production** environments run **simultaneously** on the **same server**:

```
Single Server (localhost)
│
├─ PRODUCTION (Standard Ports)
│  ├─ HTTPS: 443
│  ├─ HTTP: 8080 → 443
│  └─ DB: 5432
│
└─ DEVELOPMENT (Alternative Ports)
   ├─ HTTPS: 8443
   ├─ HTTP: 8081 → 8443
   └─ DB: 5433
```

This allows:
- Testing new features in dev before production
- Running both simultaneously
- Easy rollback if issues occur
- Training without affecting live data

---

## Architecture

### System Components

```
┌────────────────────────────────────────────────────────────────────┐
│                         EXPENSE TRACKER                             │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  CLIENTS                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │     CLI      │  │   Web        │  │   Mobile     │             │
│  │  (Rust)      │  │   Browser    │  │   App        │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
│         │                 │                  │                     │
│         └─────────────────┼──────────────────┘                     │
│                           │ HTTPS Requests                         │
│                           ↓                                        │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │                   REVERSE PROXY                             │   │
│  │  Nginx (Alpine Linux)                                       │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │ Dev: localhost:8443 | Prod: localhost:443            │  │   │
│  │  │ Routes to upstream: http://api:8000                  │  │   │
│  │  │ SSL Termination, redirect HTTP → HTTPS               │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  └────────────────────────────────────────────────────────────┘   │
│                           │                                        │
│                           ↓ Internal (http://api:8000)             │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │                   REST API SERVER                          │   │
│  │  FastAPI (Python)                                          │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │ Routes: /api/users, /api/expenses, /api/budgets... │  │   │
│  │  │ Authentication: API Key (X-API-Key header)          │  │   │
│  │  │ Database: PostgreSQL                               │  │   │
│  │  │ ORM: SQLModel                                       │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  └────────────────────────────────────────────────────────────┘   │
│                           │                                        │
│                           ↓ SQL Queries                            │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │                    DATABASE SERVER                         │   │
│  │  PostgreSQL 15 (Alpine Linux)                             │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │ Dev: localhost:5433 | Prod: localhost:5432          │  │   │
│  │  │ Tables: users, expenses, budgets, accounts, etc.     │  │   │
│  │  │ Data Volume: ./tracker-data (persistent)            │  │   │
│  │  │ Backups: Daily automated (production)               │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
```

### Directory Structure

```
expense_tracker_app/
│
├── expense-cli/                          (CLI Application)
│   ├── src/
│   │   ├── main.rs                      (Entry point)
│   │   ├── config.rs                    (⭐ Env-specific config loading)
│   │   ├── api.rs                       (HTTP client to API)
│   │   ├── cli.rs                       (⭐ Environment flag)
│   │   ├── models.rs                    (Data structures)
│   │   └── commands/                    (CLI subcommands)
│   │
│   ├── config/
│   │   ├── config.toml.example          (Template)
│   │   ├── config_dev.toml              (Dev URLs: port 8443)
│   │   ├── config_prod.toml             (Prod URLs: port 443)
│   │   └── 🔐 .env.example              (Secret management)
│   │
│   ├── .env.dev                         (⚠️ NEVER COMMIT)
│   ├── .env.prod                        (⚠️ NEVER COMMIT)
│   └── .gitignore                       (Excludes .env files)
│
├── expenses-app-v1/                      (Development Environment)
│   ├── main.py                          (FastAPI entry point)
│   ├── models.py                        (SQLModel definitions)
│   ├── services.py                      (Business logic)
│   ├── docker-compose.yaml              (⭐ Dev ports: 8443, 5433)
│   │
│   ├── nginx/
│   │   ├── nginx.conf                   (Reverse proxy config)
│   │   └── ssl/
│   │       ├── cert.pem                 (Self-signed cert)
│   │       └── key.pem                  (Private key)
│   │
│   ├── tracker-data/                    (Database volume)
│   ├── 🔐 .env                          (⚠️ NEVER COMMIT)
│   ├── .env.example                     (Template)
│   ├── README.md                        (Dev docs)
│   ├── .gitignore                       (Excludes .env)
│   └── Dockerfile                       (Build image)
│
└── app/                                  (Production Root)
    └── expense-tracker/                  (Production Environment)
        ├── docker-compose.yaml           (⭐ Prod ports: 443, 5432)
        ├── nginx/
        │   ├── nginx.conf
        │   └── ssl/
        │       ├── cert.pem              (Valid CA cert)
        │       └── key.pem               (Private key)
        │
        ├── tracker-data/                 (Database volume)
        ├── 🔐 .env                       (⚠️ NEVER COMMIT)
        ├── .env.example                  (Template)
        ├── README.md                     (Prod docs)
        ├── .gitignore                    (Excludes .env)
        └── logs/                         (Application logs)

🔐 = Secrets (never in version control)
⭐ = Key architectural component
```

---

## Networking

### Docker Bridge Networks

Each Docker Compose project creates its **own isolated network**:

```
┌─────────────────────────────────────────────────────────────────┐
│                        SERVER: localhost                         │
│                                                                  │
│  ┌──────────────────────────┐   ┌──────────────────────────┐   │
│  │ DEV NETWORK (172.18.x)   │   │ PROD NETWORK (172.19.x) │   │
│  │ expenses-app-v1_default  │   │ expense-tracker_default │   │
│  │                          │   │                          │   │
│  │ Services:                │   │ Services:                │   │
│  │  - nginx                 │   │  - nginx                 │   │
│  │  - api                   │   │  - api                   │   │
│  │  - db                    │   │  - db                    │   │
│  │                          │   │                          │   │
│  │ Port Mapping:            │   │ Port Mapping:            │   │
│  │  8443:443 (nginx)        │   │  443:443 (nginx)         │   │
│  │  8081:80 (nginx)         │   │  8080:80 (nginx)         │   │
│  │  5433:5432 (db)          │   │  5432:5432 (db)          │   │
│  └──────────────────────────┘   └──────────────────────────┘   │
│                                                                  │
│  ISOLATION RULES:                                                │
│  ✅ Containers in same network can communicate via service names│
│  ❌ Containers in different networks CANNOT communicate         │
│  ✅ External clients can only access via host ports             │
│  ❌ Internal ports (8000) not exposed to host                   │
└─────────────────────────────────────────────────────────────────┘
```

### DNS Resolution (Service Discovery)

Docker provides **automatic DNS resolution** within networks:

```
Inside DEV network:
  nginx → api:8000
  ↓
  Docker DNS: api → 172.18.0.3
  ↓
  Connection established to nginx → 172.18.0.3:8000

Inside PROD network:
  nginx → api:8000
  ↓
  Docker DNS: api → 172.19.0.3
  ↓
  Connection established to nginx → 172.19.0.3:8000

From external (CLI/Browser):
  CLI → localhost:8443
  ↓
  Host NAT: localhost:8443 → 172.18.0.4:443 (dev nginx)
  ↓
  Nginx → api:8000 (internal service name)
  ↓
  Docker DNS: api → 172.18.0.3
  ↓
  API processes request
```

### Port Mapping Strategy

**Why different ports?**

```
❌ WRONG: Both environments using same port
┌─────────────────────────┐
│ Host Port 443           │
├───────────────┬─────────┤
│ Prod: 443     │ Dev: 443│ ← PORT CONFLICT!
└───────────────┴─────────┘
Only ONE process can bind!

✅ CORRECT: Different host ports
┌─────────────────────────┐
│ Host Ports              │
├───────────┬─────────────┤
│ Prod: 443 │ Dev: 8443 ✓ │
└───────────┴─────────────┘
Both can run simultaneously!
```

**Port allocation table:**

| Resource | Container | Dev Host | Prod Host | Purpose |
|----------|-----------|----------|-----------|---------|
| nginx HTTP | 80 | 8081 | 8080 | Redirect to HTTPS |
| nginx HTTPS | 443 | 8443 | 443 | Client access |
| API | 8000 | — | — | Internal only |
| PostgreSQL | 5432 | 5433 | 5432 | Database access |

---

## Security

### Secret Management

**The Problem:**

```rust
// ❌ WRONG - Secrets exposed in code
#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,  // = "pk_live_abc123"
}

// In config file:
// api_key = "pk_live_abc123"  ← Committed to Git!
```

**The Solution:**

```rust
// ✅ CORRECT - Secrets from environment
impl Config {
    pub fn load_for_env(env: Environment) -> Result<Self> {
        let mut config: Config = load_from_toml()?;
        
        // Override from environment variables
        if let Ok(api_key) = std::env::var("API_KEY_DEV") {
            config.api.api_key = api_key;  ← From .env (not in Git!)
        }
        
        Ok(config)
    }
}
```

### File Structure for Secrets

```
Repository (Committed to Git):
├── config_dev.toml           ✅ Safe (no secrets, just URLs)
├── config_prod.toml          ✅ Safe (no secrets, just URLs)
├── .env.example              ✅ Safe (template, no values)
├── .gitignore                ✅ Marks .env as ignored
└── docker-compose.yaml       ✅ References ${API_KEY} from .env

Local Filesystem (Never in Git):
├── config/config.toml        ⚠️  Legacy (keep for compat)
├── .env.dev                  🔐 SECRETS - chmod 600
└── .env.prod                 🔐 SECRETS - chmod 600

Production Container:
├── /app/main.py              ✅ Code (no secrets)
└── Environment: API_KEY=...  🔐 Injected at runtime
```

### API Key Rotation

Recommended practice:

```bash
# Every 90 days or on team changes:

# 1. Generate new key
NEW_KEY=$(openssl rand -base64 32)

# 2. Update in production
docker compose exec api python -c "
from models import APIKey
APIKey.create(key='$NEW_KEY', active=True)
"

# 3. Update .env
sed -i "s/API_KEY=.*/API_KEY=$NEW_KEY/" .env

# 4. Restart services
docker compose restart api

# 5. Keep old key active for 24hrs (grace period)
# Then deactivate: APIKey.deactivate(old_key)
```

---

## Deployment

### CI/CD Pipeline

```
Developer pushes to main
  │
  ├─ GitHub Actions triggered
  │  ├─ Run pytest
  │  ├─ Build Docker image
  │  ├─ Push to ghcr.io/raswanth-u/expense-tracker-api:latest
  │  └─ Create deployment task
  │
  └─ Deployment Step
     ├─ SSH via Tailscale (encrypted VPN tunnel)
     ├─ SSH into /app/expense-tracker
     ├─ docker compose pull  (get latest image)
     ├─ docker compose down  (stop old containers)
     ├─ docker compose up -d (start new version)
     ├─ curl health endpoint (verify working)
     └─ Notification
        ├─ ✅ Success: Deployment complete
        └─ ❌ Failure: Health check failed, logs attached
```

### Deployment Files

```
.github/workflows/
├── ci.yml           (Run tests, linting)
└── cd.yml           (Build, push, deploy)
```

See [cd.yml](.github/workflows/cd.yml) for implementation.

### Rollback Procedure

```bash
# If new deployment has issues:

# 1. Get previous image tag
docker compose logs api | grep "image:" | tail -5

# 2. Revert to previous version
docker compose down
# Edit docker-compose.yaml:
# image: ghcr.io/raswanth-u/expense-tracker-api:v1.2.3
docker compose up -d

# 3. Verify health
curl -k https://localhost/health

# 4. Investigate issue in dev first
# Then deploy fix
```

---

## Key Technologies

| Component | Technology | Version | Purpose |
|-----------|-----------|---------|---------|
| Reverse Proxy | Nginx | Alpine | SSL termination, routing |
| API | FastAPI | 0.104+ | REST endpoints, business logic |
| ORM | SQLModel | - | Type-safe database access |
| Database | PostgreSQL | 15-alpine | Data persistence |
| CLI | Rust | 1.70+ | Command-line interface |
| Container Runtime | Docker | 24+ | Containerization |
| Orchestration | Docker Compose | 2.0+ | Multi-container coordination |
| CI/CD | GitHub Actions | - | Automated testing & deployment |
| VPN | Tailscale | - | Secure access to production |

---

## Performance Considerations

### Database Optimization

```sql
-- Connection pooling (API container)
-- SQLALCHEMY_POOL_SIZE = 10
-- SQLALCHEMY_MAX_OVERFLOW = 20

-- Indexes for common queries
CREATE INDEX idx_expenses_user_date ON expenses(user_id, created_at);
CREATE INDEX idx_budgets_month ON budgets(user_id, month);

-- Monitor slow queries
SELECT query, mean_time FROM pg_stat_statements ORDER BY mean_time DESC;
```

### Container Resource Limits

```yaml
# docker-compose.yaml
services:
  api:
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 512M
```

### Scaling Considerations

For production growth:

```yaml
# Future: Kubernetes deployment
# api:
#   replicas: 3
#   resources:
#     requests: {cpu: 0.5, memory: 512Mi}
#     limits: {cpu: 1.0, memory: 1Gi}

# Load balancer (future)
# nginx -> [api-1, api-2, api-3]

# Read replicas (future)
# main db -> read replicas
```

---

## Development Workflow

### Adding a New Feature

```bash
# 1. Start development environment
cd /home/life/projects/expense_tracker_app/expenses-app-v1
docker compose up -d

# 2. Make changes to API code
vim main.py

# 3. Rebuild and test
docker compose build api
docker compose up -d
curl -k https://localhost:8443/api/health

# 4. Test with CLI
expense --env dev user list

# 5. Commit and push
git commit -am "feat: new feature"
git push origin main

# 6. Automated deployment to production
# (CI/CD pipeline handles this)
```

### Database Migrations

```bash
# Add new column (Alembic example)
docker compose exec api alembic revision -m "Add column"

# Edit migration file
vim migrations/versions/XXXX_add_column.py

# Test locally
docker compose exec api alembic upgrade head

# Commit changes
git commit -am "db: add column migration"
```

---

## Troubleshooting Guide

### Common Issues

| Issue | Diagnosis | Solution |
|-------|-----------|----------|
| Can't connect to API | Check `docker compose ps` | Ensure containers running |
| 502 Bad Gateway | API not responding | Restart nginx: `docker compose restart nginx` |
| Database connection error | Check DATABASE_URL | Verify .env file |
| Port already in use | `lsof -i :443` | Kill process or use different port |
| SSL certificate errors | Check cert expiry | Regenerate: `openssl req -x509 ...` |

### Debug Commands

```bash
# View all services
docker compose ps

# Check specific service
docker compose logs api -f

# Database operations
docker compose exec db psql -U expense_admin -d expenses_db

# Test connectivity
docker compose exec api curl http://localhost:8000/health

# View resource usage
docker stats
```

---

## References

- **CLI Documentation**: [expense-cli/README.md](expense-cli/README.md)
- **Dev Environment**: [expenses-app-v1/README.md](expenses-app-v1/README.md)
- **Production Environment**: [app/expense-tracker/README.md](app/expense-tracker/README.md)
- **Docker Documentation**: https://docs.docker.com/
- **FastAPI**: https://fastapi.tiangolo.com/
- **PostgreSQL**: https://www.postgresql.org/docs/

---

**Last Updated**: January 2, 2026  
**Maintained By**: Development Team  
**Security Review**: Quarterly
