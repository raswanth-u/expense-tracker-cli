# Integration Testing Guide for Expense Tracker

## Overview

This document provides a comprehensive guide on performing integration tests for the Expense Tracker application. Integration testing validates that the frontend CLI (Rust) correctly interacts with the backend API (FastAPI/Python) through real HTTP requests.

## Architecture

```
┌─────────────────────┐     HTTPS      ┌─────────────────────┐
│  Rust CLI           │ ──────────────▶│  Nginx (TLS)        │
│  (expense-cli)      │                │  Port 8443          │
└─────────────────────┘                └──────────┬──────────┘
                                                  │ HTTP
                                                  ▼
                                       ┌─────────────────────┐
                                       │  FastAPI Backend    │
                                       │  (expenses-app-v1)  │
                                       │  Port 8000          │
                                       └──────────┬──────────┘
                                                  │
                                                  ▼
                                       ┌─────────────────────┐
                                       │  PostgreSQL DB      │
                                       │  Port 5433          │
                                       └─────────────────────┘
```

## Prerequisites

### 1. Docker Environment
```bash
# Verify Docker is running
docker ps

# Required containers:
# - expenses-app-v1-api-1 (FastAPI backend)
# - expenses-app-v1-nginx-1 (Nginx TLS proxy)
# - expenses-app-v1-db-1 (PostgreSQL database)
```

### 2. Python Environment
```bash
# Create virtual environment
python3 -m venv venv
source venv/bin/activate

# Install test dependencies
pip install pytest pytest-cov requests
```

### 3. Rust CLI Binary
```bash
# Build the CLI in release mode
cd expense-cli
cargo build --release

# Verify binary exists
ls -la target/release/expense-cli
```

## Setting Up Coverage-Enabled Backend

### Step 1: Modify entrypoint.sh
Add coverage support to the backend container:

```bash
#!/bin/bash
set -e

# Wait for database
echo "⏳ Waiting for database..."
sleep 5

if [ "$RUN_COVERAGE" = "true" ]; then
    echo "📊 Running with coverage enabled..."
    exec coverage run --source=main,models,services,utils -m uvicorn main:app --host 0.0.0.0 --port 8000
else
    echo "🚀 Starting FastAPI application..."
    exec uvicorn main:app --host 0.0.0.0 --port 8000
fi
```

### Step 2: Modify Dockerfile
Add coverage installation:

```dockerfile
# Install coverage
RUN pip install --no-cache-dir coverage
```

### Step 3: Rebuild and Start Containers
```bash
cd expenses-app-v1

# Stop existing containers
docker compose down

# Rebuild with coverage
docker compose build --no-cache

# Start with coverage enabled
RUN_COVERAGE=true docker compose up -d
```

## Running Integration Tests

### Basic Test Run
```bash
cd expense-cli/tests
source /path/to/venv/bin/activate
python -m pytest -v
```

### Run Specific Test File
```bash
python -m pytest test_users.py -v
```

### Run Specific Test Class
```bash
python -m pytest test_users.py::TestUserAdd -v
```

### Run Specific Test
```bash
python -m pytest test_users.py::TestUserAdd::test_add_user_basic -v
```

## Checking Coverage

### View Coverage Report
```bash
docker exec expenses-app-v1-api-1 coverage report
```

### View Missing Lines
```bash
docker exec expenses-app-v1-api-1 coverage report --show-missing
```

### Generate HTML Report
```bash
# Copy coverage data from container
docker cp expenses-app-v1-api-1:/code/.coverage .coverage.integration

# Generate HTML report
coverage html --data-file=.coverage.integration
```

## Test Structure

### conftest.py - Test Configuration
```python
# Key fixtures
@pytest.fixture
def test_user():
    """Create and return a test user."""
    result = run_cli("user", "add", "--name", "Test User", ...)
    return result["user"]

@pytest.fixture
def test_account(test_user):
    """Create and return a test savings account."""
    result = run_cli("account", "add", ...)
    return result["account"]

@pytest.fixture(autouse=True)
def clean_database():
    """Clean database before each test."""
    clear_all_data()
    yield
```

### Test File Organization
```
expense-cli/tests/
├── conftest.py          # Shared fixtures and utilities
├── test_users.py        # User CRUD operations
├── test_expenses.py     # Expense CRUD operations
├── test_budgets.py      # Budget operations
├── test_cards.py        # Credit card operations
├── test_debit.py        # Debit card operations
├── test_accounts.py     # Savings account operations
├── test_goals.py        # Savings goal operations
├── test_recurring.py    # Recurring expenses
├── test_reports.py      # Report generation
├── test_search.py       # Search functionality
├── test_dashboard.py    # Dashboard API
├── test_assets.py       # Asset management
├── test_backup.py       # Backup/restore
└── test_api_direct.py   # Direct API tests
```

## Writing New Tests

### CLI-Based Test
```python
def test_add_expense(self, test_user):
    """Test adding an expense via CLI."""
    result = run_cli(
        "expense", "add",
        "--user-id", str(test_user["id"]),
        "--amount", "50.00",
        "--category", "Food",
        "--date", "2026-01-15"
    )
    
    assert result["success"] is True
    assert result["expense"]["amount"] == 50.0
```

### Direct API Test
```python
def test_api_endpoint(self, test_user):
    """Test API endpoint directly."""
    headers = {"X-API-Key": API_KEY}
    url = f"{API_URL}/endpoint/"
    resp = requests.get(url, headers=headers, verify=False)
    
    assert resp.status_code == 200
```

## Common Issues and Solutions

### Issue 1: Database Connection Timeout
```bash
# Solution: Wait for database to be ready
docker logs expenses-app-v1-db-1  # Check DB logs
docker exec expenses-app-v1-db-1 pg_isready -U expense_admin
```

### Issue 2: SSL Certificate Verification
```python
# Solution: Disable SSL verification for self-signed certs
requests.packages.urllib3.disable_warnings()
resp = requests.get(url, verify=False)
```

### Issue 3: Coverage Data Not Updating
```bash
# Solution: Restart container to reset coverage
docker restart expenses-app-v1-api-1
sleep 5
# Run tests again
```

## Coverage Targets

| File | Target | Current |
|------|--------|---------|
| main.py | 90% | 93% ✅ |
| models.py | 90% | 94% ✅ |
| services.py | 90% | 89% |
| utils.py | 90% | 77% |
| **Total** | 90% | 91% ✅ |

## Best Practices

1. **Clean database before each test** - Use autouse fixtures
2. **Use fixtures for common data** - test_user, test_account, etc.
3. **Test both CLI and API paths** - Cover all code paths
4. **Test error conditions** - Invalid IDs, duplicate entries
5. **Test edge cases** - December dates, zero amounts, etc.

## Continuous Integration

```yaml
# .github/workflows/integration-tests.yml
name: Integration Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Start Docker services
        run: |
          cd expenses-app-v1
          RUN_COVERAGE=true docker compose up -d
          sleep 10
      
      - name: Run tests
        run: |
          cd expense-cli/tests
          pip install pytest requests
          pytest -v
      
      - name: Check coverage
        run: |
          docker exec expenses-app-v1-api-1 coverage report
```
