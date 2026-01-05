# Secrets Management Guide

A comprehensive guide on managing secrets securely in the Expense Tracker application.

**Quick Links**: [Local Development](#local-development) | [Docker Containers](#docker-containers) | [Production](#production) | [Rotation](#key-rotation) | [Incidents](#security-incidents)

---

## Overview

### The Security Challenge

Secrets (API keys, database passwords) must be:
- ✅ **Secure**: Protected from unauthorized access
- ✅ **Separate**: Never mixed with code
- ✅ **Rotatable**: Can be changed without code changes
- ✅ **Auditable**: Track who accessed what when
- ✅ **Scalable**: Work locally and in production

### What Are "Secrets"?

Sensitive values that should NEVER be in version control:

```
✅ Secrets (must protect):
- API keys (API_KEY=...)
- Database passwords (POSTGRES_PASSWORD=...)
- SSH keys
- SSL private keys
- Encryption keys
- Third-party service tokens

❌ NOT secrets (safe to commit):
- API URLs/endpoints
- Database host/port
- Configuration parameters
- Dependencies list
- Templates and examples
```

---

## Local Development

### Setup for First-Time Users

#### Step 1: Clone Repository

```bash
git clone https://github.com/your-repo/expense-tracker.git
cd expense-tracker
```

#### Step 2: Create .env Files

```bash
# In expense-cli directory
cd expense-cli
cp .env.example .env.dev
cp .env.example .env.prod

# In dev environment directory
cd ../expenses-app-v1
cp .env.example .env

# In production directory
cd ../../app/expense-tracker
cp .env.example .env
```

#### Step 3: Fill in Secret Values

```bash
# Development
nano /home/life/projects/expense_tracker_app/expense-cli/.env.dev

# Example values (these are examples - use REAL values!)
API_KEY_DEV=dev-key-xyz-123
SKIP_SSL_VERIFY_DEV=true

# Production
nano /home/life/projects/expense_tracker_app/expense-cli/.env.prod

API_KEY_PROD=prod-key-abc-789
SKIP_SSL_VERIFY_PROD=false
```

#### Step 4: Set File Permissions

```bash
# Only owner can read/write (chmod 600)
chmod 600 /home/life/projects/expense_tracker_app/expense-cli/.env.dev
chmod 600 /home/life/projects/expense_tracker_app/expense-cli/.env.prod
chmod 600 /home/life/projects/expense_tracker_app/expenses-app-v1/.env
chmod 600 /app/expense-tracker/.env

# Verify permissions
ls -la .env*
# Expected: -rw------- 1 user user

# If wrong, fix it:
chmod 600 .env*
```

#### Step 5: Verify .gitignore

```bash
# Check .env is ignored
git status | grep ".env"
# Should show nothing!

# Verify .gitignore has .env
grep ".env" .gitignore

# Check git won't accidentally commit
git ls-files | grep ".env"
# Should show nothing!
```

### Local Usage Patterns

#### Pattern 1: Load Before Running CLI

```bash
# Load secrets into environment
source /home/life/projects/expense_tracker_app/expense-cli/.env.dev

# Run CLI (uses env vars)
expense --env dev user list

# Or inline
API_KEY_DEV=... expense --env dev user list
```

#### Pattern 2: Docker Compose (Automatic)

```bash
# Docker Compose automatically loads .env file
cd /home/life/projects/expense_tracker_app/expenses-app-v1

# .env is automatically loaded by Docker Compose
docker compose up -d

# Verify it loaded
docker compose exec api env | grep API_KEY
# Output: API_KEY=value_from_.env
```

#### Pattern 3: IDE Integration (Optional)

Many IDEs support .env files:

```bash
# VS Code: Install "DotENV" extension
# PyCharm: Automatic .env support
# Vim: Use vim-dotenv plugin

# Then IDE automatically loads variables for debugging
```

---

## Docker Containers

### How Docker Gets Secrets

#### Method 1: env_file (Recommended for Compose)

```yaml
# docker-compose.yaml
services:
  api:
    env_file:
      - .env        # ← Automatically loads all variables
    environment:
      - DATABASE_URL=${DATABASE_URL}    # Reference them
      - API_KEY=${API_KEY}
```

**Advantages**:
- Simple and convenient
- Works with local .env files
- Clean separation from docker-compose.yaml

**File location**: `.env` in same directory as docker-compose.yaml

#### Method 2: environment (Direct)

```yaml
services:
  api:
    environment:
      - DATABASE_URL=postgresql://...
      - API_KEY=${API_KEY}              # ← Reference env var
```

**Advantages**:
- Single file (no extra files)
- More explicit

**Disadvantages**:
- Secrets visible in docker-compose.yaml
- Need to source .env before running

#### Method 3: Docker Secrets (Swarm/Kubernetes)

```yaml
# Production only (Kubernetes)
apiVersion: v1
kind: Secret
metadata:
  name: api-secrets
type: Opaque
data:
  api_key: base64-encoded-value
  db_password: base64-encoded-value
```

**For Kubernetes**:
```yaml
spec:
  containers:
  - name: api
    env:
    - name: API_KEY
      valueFrom:
        secretKeyRef:
          name: api-secrets
          key: api_key
```

### Secrets in Container Runtime

Once started, containers have access to secrets via environment:

```bash
# Inside container
docker compose exec api env | grep API_KEY
# Output: API_KEY=dev-api-key-123

# Python code accesses it
import os
api_key = os.getenv('API_KEY')
print(api_key)  # dev-api-key-123
```

### NOT Checking Secrets in Logs

```bash
# ❌ WRONG - Secrets exposed in logs!
docker compose logs api
# Output: "[INFO] Connecting with API_KEY=abc123xyz..."

# ✅ CORRECT - Mask secrets
# In code: Never log secrets
logger.info(f"Connecting to API")  # ← No secret!
logger.debug(f"Using key: {api_key[:4]}***")  # ← Masked!
```

---

## Production

### Secrets Management at Scale

#### For Small Teams (Current Setup)

**Using .env files**:

```bash
# 1. Secure file storage
chmod 600 .env
ls -l .env    # Verify permissions

# 2. Restricted access
# File permissions prevent non-owner access
# Group permissions = empty
# World permissions = empty

# 3. Rotation
# When key expires:
1. Generate new key
2. Update .env file
3. Restart containers
# Old key revoked immediately

# 4. Audit
# Check who has accessed server
last -f /var/log/wtmp
sudo journalctl -f
```

#### For Medium/Large Teams (Recommended)

**Using HashiCorp Vault**:

```bash
# 1. Store secrets in Vault (not in files)
vault write secret/expense-tracker-prod \
  api_key=production-key-xyz \
  db_password=secure-password

# 2. Containers fetch at startup
# (Vault agent injects secrets)

# 3. Automatic rotation
# Vault rotates database passwords daily
# Container automatically picks up new password

# 4. Audit trail
vault audit enable file file_path=/var/log/vault.log
# All access logged: who, when, what
```

**Using AWS Secrets Manager**:

```bash
# 1. Store secrets
aws secretsmanager create-secret \
  --name expense-tracker/prod/api-key \
  --secret-string "production-key-xyz"

# 2. Container fetches at startup
# (Using AWS IAM role)
import boto3
sm = boto3.client('secretsmanager')
secret = sm.get_secret_value(SecretId='expense-tracker/prod/api-key')
api_key = secret['SecretString']

# 3. Automatic rotation
# AWS manages password rotation
# Updates happen transparently

# 4. Audit trail
# CloudTrail logs all access
```

### Production Best Practices

```
✅ DO:
- Use secrets manager (Vault, AWS Secrets Manager, Azure Key Vault)
- Enable MFA for production access
- Rotate keys every 90 days
- Use separate credentials for each environment
- Encrypt data at rest
- Log all access attempts
- Restrict network access (use VPN)
- Use strong, random passwords (openssl rand -base64 32)
- Monitor for unauthorized access
- Have incident response plan

❌ DON'T:
- Store secrets in .env files (in production!)
- Commit any secrets to Git
- Use hardcoded secrets in code
- Share secrets via email/Slack
- Use default passwords
- Disable SSL verification
- Log secret values
- Store backups with secrets
- Mix dev and prod credentials
- Ignore security alerts
```

---

## Key Rotation

### When to Rotate

- **Quarterly** (every 90 days): Routine rotation
- **Immediately** if:
  - Key suspected compromised
  - Team member leaves
  - Security incident occurs
  - Key leaked/exposed in logs
  - Different vendor/tool requires new key

### Rotation Procedure

#### Step 1: Generate New Key

```bash
# Generate random API key
NEW_KEY=$(openssl rand -base64 32)
echo $NEW_KEY

# Example output:
# rT8vL9pK2xZ4cH1bN6mW5sJ3dF7gQ0eO2aK4lM

# Or use UUID
NEW_KEY=$(uuidgen)
echo $NEW_KEY
```

#### Step 2: Update in .env

```bash
# For development
# .env.dev
API_KEY_DEV=rT8vL9pK2xZ4cH1bN6mW5sJ3dF7gQ0eO2aK4lM

# For production
# .env (never commit!)
API_KEY=rT8vL9pK2xZ4cH1bN6mW5sJ3dF7gQ0eO2aK4lM
```

#### Step 3: Update in Container

```bash
# Restart services to pick up new value
docker compose down
docker compose up -d

# Verify new key is loaded
docker compose exec api env | grep API_KEY
# Output: API_KEY=rT8vL9pK2xZ4cH1bN6mW5sJ3dF7gQ0eO2aK4lM
```

#### Step 4: Test Functionality

```bash
# Verify API works with new key
curl -k https://localhost:8443/health -H "X-API-Key: rT8vL9pK2xZ4cH1bN6mW5sJ3dF7gQ0eO2aK4lM"

# Test CLI
expense --env dev user list
```

#### Step 5: Communicate & Document

```bash
# Notify team
# "API key rotated on 2026-01-02 at 14:00 UTC"
# "All services working normally"

# Update documentation
# Update any runbooks/playbooks that reference the key
```

### Graceful Transitions

For critical services, support **multiple keys** temporarily:

```python
# In API code (main.py)
VALID_KEYS = [
    "new-key-2026-01",    # Current (added today)
    "old-key-2025-10",    # Previous (valid for 24 hours)
]

def verify_api_key(header_key):
    if header_key in VALID_KEYS:
        return True
    raise InvalidAPIKey()
```

This allows:
1. Deploy new key Monday
2. Old key still works for 24 hours (clients update)
3. Revoke old key Tuesday
4. No downtime!

---

## Security Incidents

### If API Key is Compromised

**IMMEDIATELY:**

```bash
# 1. Rotate the key (right now!)
NEW_KEY=$(openssl rand -base64 32)

# 2. Update .env
sed -i "s/API_KEY=.*/API_KEY=$NEW_KEY/" .env

# 3. Restart services
docker compose down
docker compose up -d

# 4. Verify health
curl -k https://localhost/health

# Timeline: 5-10 minutes total
```

**NEXT 1 HOUR:**

```bash
# 5. Review logs for suspicious activity
docker compose logs api --since 1h | grep -i error

# 6. Check for data access
# Query logs for any unauthorized queries

# 7. Notify team/users
# "We've rotated the API key due to [reason]"
# "No user data was compromised"
```

**WITHIN 24 HOURS:**

```bash
# 8. Post-incident review
# - How was it compromised?
# - How to prevent next time?
# - Document findings

# 9. Update security procedures
# - Tighten .env file permissions?
# - Add additional monitoring?
# - Update secrets management?
```

### If Database Password is Compromised

```bash
# 1. Change PostgreSQL password
docker compose exec db psql -U expense_admin -d expenses_db -c "
ALTER USER expense_admin WITH PASSWORD 'new_secure_password';
"

# 2. Update DATABASE_URL in .env
sed -i "s/password@/new_secure_password@/" DATABASE_URL
# Before: postgresql://admin:old_pass@db:5432/expenses_db
# After:  postgresql://admin:new_pass@db:5432/expenses_db

# 3. Restart containers
docker compose down
docker compose up -d

# 4. Monitor database activity
docker compose exec db psql -U expense_admin -c "
SELECT datname, count(*) FROM pg_stat_activity GROUP BY datname;
"
```

### If .env File Exposed

**If accidentally committed to Git** (WORST CASE):

```bash
# 1. ROTATE ALL KEYS IMMEDIATELY

# 2. Remove from Git history (permanent)
# Option A: Force push (loses history)
git reset --soft HEAD~1
git reset HEAD .env
git checkout .env
git commit --amend
git push --force-with-lease

# Option B: BFG tool (recommended)
# Removes from entire history
bfg --delete-files .env
git reflog expire --expire=now --all
git gc --prune=now --aggressive

# 3. Rotate all secrets
# - Generate new API keys
# - Generate new database passwords
# - Update CI/CD secrets
# - Restart all services

# 4. Security audit
# - Assume attacker has old credentials
# - Check for unauthorized access
# - Monitor logs intensely for 7 days
```

---

## Secrets Checklist

### Before Committing

- [ ] Run `git status` - no .env files shown
- [ ] Run `grep -r "password" .` - no secrets found
- [ ] Run `grep -r "api_key" .` - no secrets found
- [ ] Check config files - no hardcoded secrets
- [ ] All secrets in .env.example (without values)

### Before Deploying

- [ ] .env file created with real values
- [ ] .env permissions set to 600
- [ ] API key is production-grade (not dev key!)
- [ ] Database password is strong (32+ chars)
- [ ] SSL certificate is valid (not self-signed)
- [ ] Secrets manager configured (if using)
- [ ] Backup of current secrets (encrypted, secure location)

### After Incident

- [ ] All compromised keys rotated
- [ ] .env file checked in code history
- [ ] Team notified
- [ ] Logs reviewed for suspicious activity
- [ ] Documentation updated
- [ ] Post-incident review completed

---

## Tools & References

### CLI Tools for Secrets

```bash
# Generate secure random strings
openssl rand -base64 32
openssl rand -hex 32
uuidgen

# Encrypt sensitive files
openssl enc -aes-256-cbc -in .env -out .env.enc

# Check Git history for secrets
git log -S "password" --oneline
git log -p | grep -i "secret\|password\|api_key"

# Scan repository for exposed secrets
# Tool: TruffleHog
docker run -it truffleHog3 filesystem . --json

# Tool: GitGuardian CLI
ggshield secret scan repo .
```

### Secrets Management Solutions

| Tool | Type | Best For | Cost |
|------|------|----------|------|
| .env files | Local | Development | Free |
| HashiCorp Vault | Enterprise | Medium/Large teams | Free/Paid |
| AWS Secrets Manager | Cloud | AWS users | ~$0.40/secret/month |
| Azure Key Vault | Cloud | Azure users | ~$0.30/secret/month |
| 1Password / LastPass | Password Manager | Team sharing | $3-$6/user/month |
| GitHub Secrets | CI/CD | GitHub Actions | Included (free) |
| GitLab Secrets | CI/CD | GitLab CI | Included (free) |

### Reading Material

- [OWASP: Sensitive Data Exposure](https://owasp.org/www-project-top-ten/)
- [12-Factor App: Store Config in Environment](https://12factor.net/config)
- [GitHub: Managing Your Secrets Safely](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [HashiCorp Vault Documentation](https://www.vaultproject.io/docs)

---

## Quick Reference

```bash
# Generate new key
openssl rand -base64 32

# Set permissions
chmod 600 .env .env.dev .env.prod

# Check permissions
ls -la .env*
# Should show: -rw------- (600)

# Load .env manually
source .env.dev

# Check loaded variables
env | grep API_KEY

# Find secrets in code
grep -r "password\|api_key\|secret" --include="*.py" --include="*.rs" --include="*.yaml"

# Verify .env in .gitignore
grep ".env" .gitignore

# Check git won't commit .env
git status | grep .env
# Should show: nothing

# Rotate key
# 1. Generate: openssl rand -base64 32
# 2. Update: nano .env
# 3. Restart: docker compose down && docker compose up -d
# 4. Verify: docker compose exec api env | grep API_KEY
```

---

**Last Updated**: January 2, 2026  
**Created By**: Security Team  
**Review Cycle**: Quarterly  
**Next Review**: April 2, 2026
