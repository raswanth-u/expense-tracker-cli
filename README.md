# Expense Tracker CLI

A powerful command-line interface for managing family expenses, budgets, and finances. The CLI supports both **development** and **production** environments running simultaneously on the same server.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Environment Setup](#environment-setup)
3. [Docker Networking Guide](#docker-networking-guide)
4. [Security & Secrets Management](#security--secrets-management)
5. [CLI Usage](#cli-usage)
6. [Configuration](#configuration)
7. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Prerequisites

- Rust 1.70+
- Docker & Docker Compose
- Python 3.8+ (for display module)

### Basic Setup

```bash
# 1. Copy example configs
cp config/config.toml.example config/config_dev.toml
cp config/config.toml.example config/config_prod.toml

# 2. Copy example env files
cp .env.example .env.dev
cp .env.example .env.prod

# 3. Edit configurations with your API keys
nano config/config_dev.toml
nano config/config_prod.toml

# 4. Build the CLI
cargo build --release

# 5. Use the CLI
./target/release/expense-cli --env dev user list
./target/release/expense-cli --env prod --interactive
```

---

## Environment Setup

### Understanding Dev vs Prod Environments

Both environments run on the **same server** but use **different ports** to avoid conflicts.

#### Development Environment
- **Location**: `/home/life/projects/expense_tracker_app/expenses-app-v1`
- **HTTPS Port**: `8443` (avoiding conflict with production)
- **HTTP Port**: `8081` (redirects to HTTPS)
- **Database Port**: `5433` (avoiding conflict with production)
- **API URL**: `https://localhost:8443/api`
- **Purpose**: Testing new features, staging environment

#### Production Environment
- **Location**: `/app/expense-tracker`
- **HTTPS Port**: `443` (standard)
- **HTTP Port**: `8080` (redirects to HTTPS)
- **Database Port**: `5432` (standard)
- **API URL**: `https://localhost/api`
- **Purpose**: Live/real data, main application

### Starting Both Environments

```bash
# Terminal 1: Start PRODUCTION
cd /app/expense-tracker
docker compose up -d
echo "✓ Production running at https://localhost/health"

# Terminal 2: Start DEVELOPMENT  
cd /home/life/projects/expense_tracker_app/expenses-app-v1
docker compose up -d
echo "✓ Development running at https://localhost:8443/health"
```

### Verify Environments

```bash
# Check production health
curl -k https://localhost/health

# Check development health
curl -k https://localhost:8443/health

# View running containers
docker ps --format "table {{.Names}}\t{{.Ports}}"
```

---

## Docker Networking Guide

### Why Different Ports?

Docker containers need unique host ports because:

```
PROBLEM: Both environments trying to use same port
┌──────────────────────────────────────────────┐
│ Port 5432 on Host (only ONE process allowed)│
├──────────┬──────────────────────────────────┤
│ Prod DB  │ ❌ CONFLICT - Can't bind!        │
│ Dev DB   │ ❌ Can't start                   │
└──────────┴──────────────────────────────────┘

SOLUTION: Use different host ports
┌──────────────────────────────────────────────┐
│ Host (localhost)                             │
├──────────┬──────────────────────────────────┤
│ :443     │ Production nginx                  │
│ :8443    │ Development nginx ✓              │
│ :5432    │ Production PostgreSQL ✓          │
│ :5433    │ Development PostgreSQL ✓         │
└──────────┴──────────────────────────────────┘
```

### Network Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    SINGLE SERVER (localhost)                         │
│                                                                      │
│  ┌──────────────────────────────┐  ┌──────────────────────────────┐ │
│  │  PRODUCTION NETWORK          │  │  DEVELOPMENT NETWORK         │ │
│  │  (expense-tracker_default)   │  │  (expenses-app-v1_default)   │ │
│  │  Subnet: 172.19.0.0/16       │  │  Subnet: 172.18.0.0/16       │ │
│  │                              │  │                              │ │
│  │  ┌────────┐  ┌────┐  ┌─────┐│  │  ┌────────┐  ┌────┐  ┌─────┐ │ │
│  │  │  nginx │──│api │──│  db ││  │  │  nginx │──│api │──│  db │ │ │
│  │  └────────┘  └────┘  └─────┘│  │  └────────┘  └────┘  └─────┘ │ │
│  │      ↓          ↓       ↓    │  │      ↓          ↓       ↓     │ │
│  │    :443     internal  5432   │  │    :8443    internal  5433   │ │
│  │    :8080               ↓     │  │    :8081               ↓     │ │
│  └──────────────────────────────┘  └──────────────────────────────┘ │
│                                                                      │
│  These networks are ISOLATED:                                        │
│  - Containers in PROD cannot reach containers in DEV                 │
│  - Each network has its own DNS resolution (app, db, nginx)          │
│  - Only external ports (host ports) allow communication              │
└─────────────────────────────────────────────────────────────────────┘
```

### Container-to-Container Communication

Within the same network, containers communicate using **service names**:

```yaml
# Inside docker-compose.yml
services:
  nginx:
    image: nginx:alpine
    environment:
      - BACKEND_URL=http://api:8000  # Uses service name, NOT localhost!
  
  api:
    build: .
    environment:
      - DATABASE_URL=postgresql://db:5432  # Again, service name!
  
  db:
    image: postgres:15-alpine
    # Containers connect via: postgres://db:5432
```

**Why service names work inside containers?**
- Docker's embedded DNS resolves `api` → `172.18.0.3` (or similar)
- Host localhost DNS does NOT know about container names
- External clients must use host IP + host port

### Port Mapping Deep Dive

```
External Client CLI:
  expense-cli ──[localhost:8443]──> Nginx (host listens here)
                      ↓
                Docker Network (NAT)
                      ↓
              Internal: 172.18.0.4:443 (Nginx container)
                      ↓
              Proxies to: http://api:8000 (internal service name)
                      ↓
              Internal: 172.18.0.3:8000 (API container)
                      ↓
              Proxies to: postgresql://db:5432 (internal service name)
                      ↓
              Internal: 172.18.0.2:5432 (DB container)
```

---

## Security & Secrets Management

### ⚠️ Problem: Hardcoded Secrets

**❌ WRONG** - API keys in config files:
```toml
# config_prod.toml
[api]
api_key = "pk_live_abc123xyz789"  # ❌ EXPOSED IN GIT!
```

**Why this is dangerous:**
- Secrets committed to Git history (permanent record!)
- Visible to anyone with repo access
- Hard to rotate without changing code
- Violates security compliance (SOC 2, HIPAA, etc.)

### ✅ Solution: Environment Variables + .env Files

We use the **industry-standard 12-factor app approach**:

```
.env (LOCAL ONLY)           .gitignore              Environment Variables
┌─────────────────┐        ┌────────────┐          (in containers/CI/CD)
│ API_KEY=secret  │ ──────>│ *.env ────>│────────> DATABASE_URL=postgres://...
│ DB_PASSWORD=... │        │ .env.local │          API_KEY=secret
└─────────────────┘        └────────────┘          
  (Never committed!)       (Marks .env             (Set at runtime)
                           as ignored)
```

### Setting Up Secrets

#### 1. Create `.env.dev` and `.env.prod` files

```bash
# Development
cat > /home/life/projects/expense_tracker_app/expense-cli/.env.dev << 'EOF'
# Development Environment Secrets
API_KEY_DEV=your-dev-api-key-here
SKIP_SSL_VERIFY_DEV=true
EOF

# Production
cat > /home/life/projects/expense_tracker_app/expense-cli/.env.prod << 'EOF'
# Production Environment Secrets
API_KEY_PROD=your-production-api-key-here
SKIP_SSL_VERIFY_PROD=false
EOF

chmod 600 .env.dev .env.prod  # Read/write only by owner
```

#### 2. Update `.gitignore`

```bash
# In /home/life/projects/expense_tracker_app/expense-cli/.gitignore
# Add these lines:
.env
.env.local
.env.*.local
.env.dev
.env.prod
.env.production
.env.staging
```

#### 3. Create `.env.example` (safe to commit)

```bash
cat > /home/life/projects/expense_tracker_app/expense-cli/.env.example << 'EOF'
# Copy this file to .env.dev and .env.prod
# Then fill in actual values (DO NOT commit .env files!)

# Development
API_KEY_DEV=your-dev-api-key-here
SKIP_SSL_VERIFY_DEV=true

# Production  
API_KEY_PROD=your-production-api-key-here
SKIP_SSL_VERIFY_PROD=false
EOF
```

#### 4. Update Rust config to load from env vars

Update `src/config.rs`:

```rust
impl Config {
    pub fn load_for_env(env: Environment) -> Result<Self> {
        let env_name = match env {
            Environment::Dev => "dev",
            Environment::Prod => "prod",
        };
        
        // ... existing code ...
        
        let mut config: Config = toml::from_str(&content)?;
        
        // Override API key from environment variable if present
        if let Ok(api_key) = std::env::var(format!("API_KEY_{}", env_name.to_uppercase())) {
            config.api.api_key = api_key;
            eprintln!("✓ Loaded API key from environment variable");
        }
        
        // Override skip_ssl_verify from env var if present
        if let Ok(skip_ssl) = std::env::var(format!("SKIP_SSL_VERIFY_{}", env_name.to_uppercase())) {
            config.api.skip_ssl_verify = skip_ssl.to_lowercase() == "true";
            eprintln!("✓ Loaded SSL verification setting from environment");
        }
        
        config.environment = Some(env);
        Ok(config)
    }
}
```

### Usage Patterns

#### Local Development (with .env files)

```bash
# Load secrets from .env.dev before running CLI
source .env.dev
cargo run -- --env dev user list
```

#### Docker Containers (best practice)

In your `docker-compose.yaml`:

```yaml
# .env file path gets loaded automatically by Docker Compose
env_file:
  - .env.dev          # Docker Compose loads this automatically

# Or explicit environment variables
environment:
  - API_KEY=${API_KEY_DEV}
  - DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@db:5432
```

#### CI/CD Pipeline (GitHub Actions)

```yaml
# .github/workflows/test.yml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        env:
          API_KEY_DEV: ${{ secrets.API_KEY_DEV }}
          SKIP_SSL_VERIFY_DEV: "true"
        run: cargo test
```

### Security Best Practices

```
✅ DO:
- Use .env files locally
- Use environment variables in Docker/K8s
- Use secrets manager in production (Vault, AWS Secrets Manager)
- Rotate secrets regularly
- Use .gitignore to exclude .env files
- Use different keys for dev/prod/staging
- Never log sensitive values
- Use `chmod 600` on .env files (read/write only)

❌ DON'T:
- Commit .env files to Git
- Hardcode secrets in source code
- Use same API key for all environments
- Log API keys or passwords
- Share .env files via Slack/Email
- Store secrets in comments
```

### Recommended Setup for Teams

```bash
# 1. Commit these files to Git:
- .env.example              # ✓ Safe - no real values
- .gitignore                # ✓ Marks .env as ignored
- src/config.rs             # ✓ Loads from env vars
- config/config_dev.toml    # ✓ No secrets, just URLs
- config/config_prod.toml   # ✓ No secrets, just URLs

# 2. Create locally (NEVER commit):
- .env.dev                  # ✗ Has real API keys
- .env.prod                 # ✗ Has real API keys
- config/config.toml        # ✗ Backup/old config

# 3. In CI/CD (GitHub Actions Secrets):
- API_KEY_DEV
- API_KEY_PROD
- DB_PASSWORD_DEV
- DB_PASSWORD_PROD
```

---

## CLI Usage

### Global Flags

```bash
# Environment selection (default: dev)
expense --env dev <command>
expense -e prod <command>

# JSON output (for scripting)
expense --json user list

# Interactive mode
expense --interactive
expense -i --env prod
```

### Examples

```bash
# Development environment (default)
expense user list
expense --env dev user list
expense -e dev expense add --amount 50 --category Food --user-id 1

# Production environment
expense --env prod user list
expense -e prod user add --name "John" --email "john@example.com"

# Interactive mode
expense --interactive
expense --env prod --interactive

# JSON output for scripting
expense --json expense list | jq '.[0]'
```

### Common Commands

```bash
# Users
expense user list
expense user add --name "Alice" --email "alice@example.com"
expense user view 1

# Expenses
expense expense add --amount 25.50 --category Food --user-id 1 --description "Lunch"
expense expense list --user-id 1
expense expense view 123

# Budgets
expense budget add --category Food --amount 500 --month 2026-01
expense budget status --month 2026-01

# Dashboard
expense dashboard --month 2026-01
```

For complete command reference: `expense --help`

---

## Configuration

### Config File Structure

#### Development: `config/config_dev.toml`

```toml
[api]
# Dev uses port 8443 to avoid conflict with production
base_url = "https://localhost:8443/api"
api_key = "dev-key"  # Overridden by .env.dev
skip_ssl_verify = true

[display]
python_path = ""
display_module = "./python_display/display.py"

[defaults]
default_user_id = 1
date_format = "%Y-%m-%d"
```

#### Production: `config/config_prod.toml`

```toml
[api]
# Prod uses standard port 443
base_url = "https://localhost/api"
api_key = "prod-key"  # Overridden by .env.prod
skip_ssl_verify = true  # Set to false for valid certs

[display]
python_path = ""
display_module = "./python_display/display.py"

[defaults]
default_user_id = 1
date_format = "%Y-%m-%d"
```

### Environment Variables

```bash
# Development
API_KEY_DEV=your-actual-dev-key
SKIP_SSL_VERIFY_DEV=true

# Production
API_KEY_PROD=your-actual-prod-key
SKIP_SSL_VERIFY_PROD=false  # Use real certs in prod!
```

### Loading Priority

Config values are loaded in this order (later overrides earlier):

1. TOML config file (base values)
2. Environment variables (`API_KEY_DEV`, `SKIP_SSL_VERIFY_DEV`)
3. Command-line flags (future enhancement)

---

## Building & Installing

### Build Release Binary

```bash
# Optimized build
cargo build --release

# Binary location
./target/release/expense-cli
```

### Install Globally (optional)

```bash
# Copy to system bin
sudo cp target/release/expense-cli /usr/local/bin/expense

# Or symlink
sudo ln -sf $(pwd)/target/release/expense-cli /usr/local/bin/expense

# Verify
expense --version
```

### Python Display Module

```bash
# Install dependencies
cd python_display
pip install -r requirements.txt

# Or use included venv
source cli-venv/bin/activate
```

---

## Troubleshooting

### API Connection Issues

```bash
# Check connectivity to development environment
curl -k https://localhost:8443/health

# Check connectivity to production environment
curl -k https://localhost/health

# Enable debug info
export RUST_LOG=debug
expense --env dev user list
```

### SSL Certificate Errors

```
Error: SSL certificate problem: self signed certificate

Solution: Set skip_ssl_verify = true in config file
(Development only - never in production with untrusted certs!)
```

### Config Not Loading

```bash
# Check config file exists
ls -la config/config_dev.toml
ls -la config/config_prod.toml

# Validate TOML syntax
cat config/config_dev.toml

# Check environment variables are loaded
echo $API_KEY_DEV
```

### Port Already in Use

```bash
# Find what's using the port
lsof -i :8443
lsof -i :443

# Kill process if needed
kill -9 <PID>

# Or change port in docker-compose.yaml
# ports:
#   - "9443:443"  # Changed from 8443
```

### Database Connection Issues

```bash
# Check containers are running
docker ps | grep expense

# Check database is healthy
docker compose -f expenses-app-v1/docker-compose.yaml exec db pg_isready

# View container logs
docker compose logs db
docker compose logs api
```

---

## Advanced Topics

### Using with SSH Tunneling

If running on a remote server via SSH:

```bash
# Forward remote ports to local
ssh -L 8443:localhost:8443 -L 5433:localhost:5433 user@remote-server

# Then use CLI normally
expense --env dev user list
```

### Docker Network Inspection

```bash
# List networks
docker network ls

# Inspect a network
docker network inspect expenses-app-v1_default

# Test container connectivity
docker exec -it <container> ping <service-name>
```

### Performance Tuning

For large datasets, increase connection pool:

In `src/api.rs`, modify client builder:

```rust
let client = Client::builder()
    .pool_max_idle_per_host(20)
    .timeout(Duration::from_secs(30))
    .build()?;
```

---

## Contributing

Contributions welcome! Please:

1. Use `--env dev` for testing
2. Don't commit `.env` files or secrets
3. Update config examples, not main files
4. Test with both environments when possible

---

## Support

- **Issues**: Check [Troubleshooting](#troubleshooting) section
- **Logs**: `docker compose logs api`
- **Docs**: See [NETWORKING.md](../NETWORKING.md) for detailed network info

---

**Last Updated**: January 2026
