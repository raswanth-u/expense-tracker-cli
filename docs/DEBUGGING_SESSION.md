# Integration Testing Debugging Session

## Session Summary

**Date:** January 4-5, 2026  
**Objective:** Achieve 90% code coverage on backend files (main.py, models.py, services.py, utils.py) by running frontend CLI tests against the backend API  
**Final Result:** 91% total coverage achieved (93% main.py, 94% models.py, 89% services.py, 77% utils.py)

---

## Initial State

### Starting Coverage (Backend Unit Tests)
```
main.py     98%
models.py   96%
services.py 95%
utils.py    99%
```

**Issue:** These were backend unit test coverage numbers, not integration test coverage.

---

## Debugging Steps and Solutions

### Issue 1: Understanding Integration vs Unit Testing

**Problem:** Initial confusion between unit test coverage and integration test coverage.

**Clarification:** 
- Unit tests test backend code directly
- Integration tests run frontend CLI which calls backend API
- Coverage must be measured inside the running API container

**Command to verify containers:**
```bash
docker ps
# Output: expenses-app-v1-api-1, expenses-app-v1-nginx-1, expenses-app-v1-db-1
```

---

### Issue 2: Enabling Coverage in Docker Container

**Problem:** Backend was not running with coverage enabled.

**Solution:** Modify entrypoint.sh to support RUN_COVERAGE environment variable.

**Command - Edit entrypoint.sh:**
```bash
cat > expenses-app-v1/entrypoint.sh << 'EOF'
#!/bin/bash
set -e
echo "⏳ Waiting for database..."
sleep 5

if [ "$RUN_COVERAGE" = "true" ]; then
    echo "📊 Running with coverage enabled..."
    exec coverage run --source=main,models,services,utils -m uvicorn main:app --host 0.0.0.0 --port 8000
else
    echo "🚀 Starting FastAPI application..."
    exec uvicorn main:app --host 0.0.0.0 --port 8000
fi
EOF
```

**Command - Modify Dockerfile:**
```bash
# Add to Dockerfile:
RUN pip install --no-cache-dir coverage
```

**Command - Rebuild container:**
```bash
cd expenses-app-v1
docker compose down
docker compose build --no-cache
RUN_COVERAGE=true docker compose up -d
```

**Verification:**
```bash
docker logs expenses-app-v1-api-1 --tail 10
# Should show: "📊 Running with coverage enabled..."
```

---

### Issue 3: Database Schema Mismatch

**Problem:** Tests failed with errors like:
```
column "created_at" does not exist
```

**Diagnosis Command:**
```bash
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db -c "\d expense"
```

**Root Cause:** Database tables were missing `created_at` columns and had VARCHAR date columns instead of DATE.

**Solution - Add missing columns:**
```bash
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db << 'EOF'
-- Add created_at columns
ALTER TABLE expense ADD COLUMN IF NOT EXISTS created_at TIMESTAMP DEFAULT NOW();
ALTER TABLE creditcard ADD COLUMN IF NOT EXISTS created_at TIMESTAMP DEFAULT NOW();
ALTER TABLE budget ADD COLUMN IF NOT EXISTS created_at TIMESTAMP DEFAULT NOW();

-- Convert VARCHAR to DATE
ALTER TABLE expense ALTER COLUMN date TYPE DATE USING date::DATE;
ALTER TABLE recurringexpensetemplate ALTER COLUMN start_date TYPE DATE USING start_date::DATE;
ALTER TABLE recurringexpensetemplate ALTER COLUMN end_date TYPE DATE USING end_date::DATE;
ALTER TABLE savingsgoal ALTER COLUMN deadline TYPE DATE USING deadline::DATE;
EOF
```

**Verification:**
```bash
docker exec expenses-app-v1-db-1 psql -U expense_admin -d expenses_db -c "\d expense"
```

---

### Issue 4: Nginx Container Name Mismatch

**Problem:** Nginx proxy failed to connect to API container.

**Diagnosis:**
```bash
docker logs expenses-app-v1-nginx-1
# Error: upstream not found
```

**Solution - Update nginx.conf:**
```nginx
upstream api {
    server expenses-app-v1-api-1:8000;  # Use correct container name
}
```

**Command - Restart nginx:**
```bash
docker restart expenses-app-v1-nginx-1
```

---

### Issue 5: Coverage Path Mapping

**Problem:** Coverage report showed paths like `/code/main.py` instead of local paths.

**Solution - Copy and fix coverage data:**
```bash
# Copy coverage data from container
docker cp expenses-app-v1-api-1:/code/.coverage .coverage.integration

# Fix paths in SQLite database
sqlite3 .coverage.integration << 'EOF'
UPDATE file SET path = REPLACE(path, '/code/', '/home/life/projects/expense_tracker_app/expenses-app-v1/');
EOF

# Generate report
coverage report --data-file=.coverage.integration
```

---

### Issue 6: CLI Argument Mismatches

**Problem:** Tests failed with "unexpected argument" errors.

**Example Error:**
```
error: unexpected argument '--month-of-year' found
```

**Diagnosis - Check CLI help:**
```bash
./expense-cli recurring add --help
```

**Solutions for various argument issues:**

| Wrong | Correct | Command to verify |
|-------|---------|-------------------|
| `--month-of-year` | (not supported) | `recurring add --help` |
| `--payment-method` | `--payment` | `expense add --help` |
| `--debit-card-id` | `--debit-id` | `expense add --help` |
| `--month` (on list) | `--from/--to` | `expense list --help` |
| `--months` | `--months` | `card utilization --help` |

**Fix Test Example:**
```python
# Before (wrong)
run_cli("expense", "add", "--payment-method", "credit_card", ...)

# After (correct)
run_cli("expense", "add", "--payment", "credit_card", ...)
```

---

### Issue 7: Coverage Not Updating

**Problem:** Coverage numbers didn't change after adding new tests.

**Cause:** Container caching old coverage data.

**Solution:**
```bash
# Restart container to reset coverage
docker restart expenses-app-v1-api-1

# Wait for startup
sleep 5

# Verify it's running with coverage
docker logs expenses-app-v1-api-1 --tail 5

# Run tests again
cd expense-cli/tests
python -m pytest -v
```

**Check coverage:**
```bash
docker exec expenses-app-v1-api-1 coverage report
```

---

### Issue 8: Uncovered API Endpoints

**Problem:** Some API endpoints not exposed through CLI.

**Example:** Credit card summary endpoint (`/credit-cards/summary`)

**Solution - Create direct API tests:**
```python
def api_get(endpoint: str, params: dict = None) -> dict:
    """Make a GET request to the API."""
    headers = {"X-API-Key": API_KEY}
    url = f"{API_URL}/{endpoint}"
    resp = requests.get(url, headers=headers, params=params, verify=False)
    return resp.json()

def test_all_cards_summary(self, test_user, test_credit_card):
    result = api_get("credit-cards/summary")
    assert "total_cards" in result or "cards" in result
```

---

## Test Execution Commands

### Run All Tests
```bash
cd expense-cli/tests
source /path/to/venv/bin/activate
python -m pytest -v
```

### Run Tests with Output
```bash
python -m pytest -v -s  # -s shows print statements
```

### Run Specific Test File
```bash
python -m pytest test_users.py -v
```

### Run Specific Test Class
```bash
python -m pytest test_budgets.py::TestBudgetDecember -v
```

### Run Specific Test
```bash
python -m pytest test_recurring.py::TestRecurringFrequencies::test_process_daily_recurring -v
```

---

## Coverage Commands

### Basic Coverage Report
```bash
docker exec expenses-app-v1-api-1 coverage report
```

### Show Missing Lines
```bash
docker exec expenses-app-v1-api-1 coverage report --show-missing
```

### Filter by File
```bash
docker exec expenses-app-v1-api-1 coverage report --include=services.py
```

### Generate HTML Report
```bash
docker exec expenses-app-v1-api-1 coverage html
docker cp expenses-app-v1-api-1:/code/htmlcov ./htmlcov
# Open htmlcov/index.html in browser
```

---

## Final Coverage Results

### After All Optimizations

```
Name          Stmts   Miss  Cover
---------------------------------
main.py         720     51    93%
models.py       344     19    94%
services.py     327     36    89%
utils.py         94     22    77%
---------------------------------
TOTAL          1485    128    91%
```

### Test Count Progression
| Stage | Tests | Status |
|-------|-------|--------|
| Initial | 193 | All passing |
| After frequency tests | 198 | 4 failing |
| After CLI fixes | 204 | All passing |
| After API tests | 230 | All passing |
| Final | 267 | All passing |

---

## Key Learnings

1. **Integration coverage requires measuring inside the running container**
2. **Database schema must match model definitions exactly**
3. **CLI arguments must be verified with `--help`**
4. **Some API endpoints need direct testing (not CLI)**
5. **Container restart clears coverage cache**
6. **Dead code in utils.py (unused functions) can't be covered**

---

## Remaining Uncovered Code

### utils.py (77%)
- `get_month_date_range()` - Function is never called (dead code)
- `parse_month()` error handling - Requires malformed input
- `calculate_next_occurrence()` edge cases - Specific date scenarios

### services.py (89%)
- HTTPException handlers - Require invalid/inactive resources
- Asset-savings account linking - Specific transaction flows
- Inactive resource checks - Require soft-deleted entities

### Recommendations
1. Remove unused `get_month_date_range()` function from utils.py
2. Add unit tests for edge cases that can't be triggered via API
3. Consider increasing test isolation to test error paths
