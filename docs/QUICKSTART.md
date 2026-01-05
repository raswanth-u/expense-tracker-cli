# Quick Start Guide - Multi-Environment Setup

**TL;DR**: Run dev and prod simultaneously with secrets management ✨

---

## 60-Second Setup

```bash
# 1. Clone and navigate
cd /home/life/projects/expense_tracker_app/expense-cli

# 2. Create .env files
cp .env.example .env.dev
cp .env.example .env.prod

# 3. Edit with your API keys
nano .env.dev         # Add API_KEY_DEV value
nano .env.prod        # Add API_KEY_PROD value

# 4. Set permissions
chmod 600 .env.dev .env.prod

# 5. Test CLI
export API_KEY_DEV=your-dev-key
cargo run -- --env dev user list

# Done! ✨
```

---

## 5-Minute Full Setup

```bash
# ========== TERMINAL 1: Start Production ==========
cd /app/expense-tracker

# Copy and edit .env
cp .env.example .env
nano .env
# Fill in: POSTGRES_PASSWORD, API_KEY, DATABASE_URL

chmod 600 .env

# Start
docker compose up -d

# Verify
curl -k https://localhost/health
# Expected: {"status": "ok"}

# ========== TERMINAL 2: Start Development ==========
cd /home/life/projects/expense_tracker_app/expenses-app-v1

# Copy and edit .env
cp .env.example .env
nano .env
# Fill in: POSTGRES_PASSWORD, API_KEY, DATABASE_URL

chmod 600 .env

# Start
docker compose up -d

# Verify
curl -k https://localhost:8443/health
# Expected: {"status": "ok"}

# ========== TERMINAL 3: Use CLI ==========
cd /home/life/projects/expense_tracker_app/expense-cli

# Setup CLI configs
cp config/config.toml.example config/config_dev.toml
cp config/config.toml.example config/config_prod.toml

# Create CLI env files
cp .env.example .env.dev
cp .env.example .env.prod
nano .env.dev   # Add API_KEY_DEV
nano .env.prod  # Add API_KEY_PROD

# Test dev environment
source .env.dev
expense --env dev user list

# Test prod environment
source .env.prod
expense --env prod user list

# Done! Both environments running! 🎉
```

---

## Common Commands

### Check Status

```bash
# Which containers are running?
docker ps --format "table {{.Names}}\t{{.Ports}}\t{{.Status}}"

# Check specific environment
cd expenses-app-v1 && docker compose ps
cd ../../app/expense-tracker && docker compose ps
```

### View Logs

```bash
# Dev logs
cd expenses-app-v1
docker compose logs -f api        # API logs
docker compose logs db            # Database logs
docker compose logs nginx         # Nginx logs

# Prod logs
cd ../../app/expense-tracker
docker compose logs -f api
```

### Stop/Start

```bash
# Stop development
cd expenses-app-v1
docker compose down

# Stop production
cd ../../app/expense-tracker
docker compose down

# Start again
docker compose up -d
```

### Test API

```bash
# Development
curl -k https://localhost:8443/api/docs       # Swagger UI
curl -k https://localhost:8443/health         # Health check

# Production
curl -k https://localhost/api/docs            # Swagger UI
curl -k https://localhost/health              # Health check
```

### Use CLI

```bash
# Development (default)
expense user list
expense --env dev user list
expense -e dev --interactive

# Production
expense --env prod user list
expense -e prod --interactive
```

---

## Secrets Management

### Create .env Files

```bash
# Copy template
cp .env.example .env

# Edit (add real values)
nano .env

# Secure it
chmod 600 .env

# Verify (should show -rw-------)
ls -la .env
```

### What to Put in .env

```bash
# Database
POSTGRES_USER=expense_admin
POSTGRES_PASSWORD=your_secure_password_here

# API
API_KEY=your-api-key-here
DATABASE_URL=postgresql://admin:password@db:5432/expenses_db

# Environment
ENVIRONMENT=development
LOG_LEVEL=DEBUG
```

### Never Do This

```
❌ DON'T commit .env to Git
❌ DON'T share .env via email/Slack
❌ DON'T hardcode secrets in code
❌ DON'T set .env permissions to 644 or 777
```

---

## Architecture at a Glance

```
Single Server (localhost)
│
├── PRODUCTION (443, 5432)
│   ├── nginx → api → database
│   └── Access: https://localhost/api
│
└── DEVELOPMENT (8443, 5433)
    ├── nginx → api → database
    └── Access: https://localhost:8443/api

Both running simultaneously!
```

---

## Troubleshooting

### Can't connect to API?

```bash
# 1. Check containers running
docker ps | grep nginx

# 2. Check ports available
lsof -i :443
lsof -i :8443

# 3. Check logs
docker compose logs api

# 4. Restart
docker compose down
docker compose up -d
```

### Port already in use?

```bash
# Find what's using it
lsof -i :443

# Kill if needed
kill -9 <PID>
```

### .env not loading?

```bash
# Check file exists
ls -la .env

# Check readable
cat .env | head -5

# Check permissions (should be 600)
ls -la .env
```

### CLI config not found?

```bash
# Create config files
cp config/config.toml.example config/config_dev.toml
cp config/config.toml.example config/config_prod.toml

# Check they exist
ls -la config/config_*.toml
```

---

## Verify Everything Works

```bash
# ✅ Check prod
curl -k https://localhost/health

# ✅ Check dev
curl -k https://localhost:8443/health

# ✅ Test CLI (dev)
expense --env dev user list

# ✅ Test CLI (prod)
expense --env prod user list

# ✅ Check .env not in Git
git status | grep ".env"

# ✅ Check Docker networks
docker network ls | grep expense

# All working? 🎉 You're done!
```

---

## Documentation

Need more details?

- **CLI Setup**: [expense-cli/README.md](../expense-cli/README.md)
- **Dev Environment**: [expenses-app-v1/README.md](../expenses-app-v1/README.md)
- **Prod Environment**: [app/expense-tracker/README.md](../expense-tracker/README.md)
- **Architecture**: [ARCHITECTURE.md](./ARCHITECTURE.md)
- **Secrets**: [SECRETS_MANAGEMENT.md](./SECRETS_MANAGEMENT.md)

---

## What's Different Now

### Before ❌

```
- Only prod OR dev running (not both)
- Secrets in config files (risk of exposure)
- No environment-specific CLI config
- Port conflicts if tried to run both
- Limited documentation
```

### After ✅

```
- Dev AND prod running simultaneously
- Secrets in .env (never in version control)
- CLI has --env flag for switching
- Different ports (no conflicts)
- Complete documentation
```

---

## Next Steps

1. ✅ Setup .env files (secrets)
2. ✅ Start both environments
3. ✅ Test with CLI
4. ✅ Read documentation for details
5. ✅ Deploy with confidence!

---

**Ready?** Start with the 60-second setup above! 🚀
