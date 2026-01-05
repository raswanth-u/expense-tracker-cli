# Expense CLI Tests
# 
# Run: pytest tests/ -v
# Run single file: pytest tests/test_users.py -v

import pytest
import subprocess
import json
import requests
from typing import Any

# Configuration
CLI_PATH = "/home/life/projects/expense_tracker_app/expense-cli/target/release/expense-cli"
API_URL = "https://localhost:8443/api"
API_KEY = "supersecretapikey"

# Disable SSL warnings for self-signed cert
requests.packages.urllib3.disable_warnings()

# Set environment variable for CLI to use
import os
os.environ["API_KEY_DEV"] = API_KEY


def run_cli(*args, expect_success=True) -> dict:
    """Run CLI command with --json flag and return parsed JSON output."""
    cmd = [CLI_PATH, "--json"] + list(args)
    # Run from the CLI directory so config file is found
    cli_dir = "/home/life/projects/expense_tracker_app/expense-cli"
    # Pass the environment including API_KEY_DEV
    env = os.environ.copy()
    result = subprocess.run(cmd, capture_output=True, text=True, cwd=cli_dir, env=env)
    
    if expect_success and result.returncode != 0:
        raise AssertionError(f"CLI failed: {result.stderr}\nstdout: {result.stdout}")
    
    if result.stdout.strip():
        try:
            return json.loads(result.stdout)
        except json.JSONDecodeError:
            # For non-JSON output (like --help)
            return {"raw_output": result.stdout}
    return {}


def run_cli_display(*args) -> tuple[int, str, str]:
    """Run CLI command WITHOUT --json flag (test display output)."""
    cmd = [CLI_PATH] + list(args)
    # Run from the CLI directory so config file is found
    cli_dir = "/home/life/projects/expense_tracker_app/expense-cli"
    # Pass the environment including API_KEY_DEV
    env = os.environ.copy()
    result = subprocess.run(cmd, capture_output=True, text=True, cwd=cli_dir, env=env)
    return result.returncode, result.stdout, result.stderr


def api_request(method: str, endpoint: str, data: dict = None) -> Any:
    """Direct API request for setup/cleanup."""
    headers = {"X-API-Key": API_KEY}
    url = f"{API_URL}/{endpoint}"
    
    if method == "GET":
        resp = requests.get(url, headers=headers, verify=False)
    elif method == "POST":
        resp = requests.post(url, json=data, headers=headers, verify=False)
    elif method == "PUT":
        resp = requests.put(url, json=data, headers=headers, verify=False)
    elif method == "DELETE":
        resp = requests.delete(url, headers=headers, verify=False)
    else:
        raise ValueError(f"Unknown method: {method}")
    
    if resp.status_code in (200, 201):
        try:
            return resp.json()
        except:
            return resp.text
    elif resp.status_code == 204:
        return None
    else:
        raise Exception(f"API error {resp.status_code}: {resp.text}")


def clear_all_data():
    """Clear all data from database via API (in correct order for FK constraints)."""
    # Delete in order: transactions first, then entities with FKs, then base entities
    # Note: API endpoints require trailing slashes for list endpoints
    
    # 0. First delete savings account transactions (they reference expenses)
    accounts = api_request("GET", "savings-accounts/")
    for a in accounts:
        # Get transactions for this account and delete them
        try:
            txns = api_request("GET", f"savings-accounts/{a['id']}/transactions")
            for t in txns:
                try:
                    api_request("DELETE", f"savings-accounts/{a['id']}/transactions/{t['id']}")
                except Exception:
                    pass  # Transaction may have already been deleted
        except Exception:
            pass  # Account may not have transactions endpoint
    
    # 1. Expenses (depends on users, cards, accounts)
    expenses = api_request("GET", "expenses/")
    for e in expenses:
        try:
            api_request("DELETE", f"expenses/{e['id']}")
        except Exception as ex:
            print(f"Warning: Could not delete expense {e['id']}: {ex}")
    
    # 2. Recurring templates
    recurring = api_request("GET", "recurring-expenses/")
    for r in recurring:
        try:
            api_request("DELETE", f"recurring-expenses/{r['id']}")
        except Exception:
            pass
    
    # 3. Credit card transactions (implicitly deleted with cards, but just in case)
    cards = api_request("GET", "credit-cards/")
    for c in cards:
        try:
            api_request("DELETE", f"credit-cards/{c['id']}")
        except Exception:
            pass
    
    # 4. Debit cards (depends on accounts)
    debit_cards = api_request("GET", "debit-cards/")
    for dc in debit_cards:
        try:
            api_request("DELETE", f"debit-cards/{dc['id']}")
        except Exception:
            pass
    
    # 5. Savings accounts
    accounts = api_request("GET", "savings-accounts/")
    for a in accounts:
        try:
            api_request("DELETE", f"savings-accounts/{a['id']}")
        except Exception:
            pass
    
    # 6. Savings goals
    goals = api_request("GET", "savings-goals/")
    for g in goals:
        try:
            api_request("DELETE", f"savings-goals/{g['id']}")
        except Exception:
            pass
    
    # 7. Budgets
    budgets = api_request("GET", "budgets/")
    for b in budgets:
        try:
            api_request("DELETE", f"budgets/{b['id']}")
        except Exception:
            pass
    
    # 8. Assets
    assets = api_request("GET", "assets/")
    for a in assets:
        try:
            api_request("DELETE", f"assets/{a['id']}")
        except Exception:
            pass
    
    # 9. Users (last, as everything depends on them)
    users = api_request("GET", "users/")
    for u in users:
        try:
            api_request("DELETE", f"users/{u['id']}?hard=true")
        except Exception:
            pass


@pytest.fixture(autouse=True)
def clean_database():
    """Clean database before each test."""
    clear_all_data()
    yield
    # Optionally clean after test too
    # clear_all_data()


@pytest.fixture
def test_user():
    """Create and return a test user."""
    result = run_cli("user", "add", "--name", "Test User", "--email", "test@example.com", "--role", "admin")
    assert result["success"] is True
    return result["user"]


@pytest.fixture
def test_account(test_user):
    """Create and return a test savings account."""
    result = run_cli(
        "account", "add",
        "--name", "Test Checking",
        "--bank", "Test Bank",
        "--last-four", "1234",
        "--user-id", str(test_user["id"]),
        "--account-type", "checking",
        "--balance", "1000",
        "--min-balance", "100",
        "--interest-rate", "2.5"
    )
    assert result["success"] is True
    return result["account"]


@pytest.fixture
def test_credit_card(test_user):
    """Create and return a test credit card."""
    result = run_cli(
        "card", "add",
        "--name", "Test Card",
        "--last-four", "5678",
        "--limit", "5000",
        "--user-id", str(test_user["id"]),
        "--billing-day", "15"
    )
    assert result["success"] is True
    return result["card"]


@pytest.fixture
def test_debit_card(test_user, test_account):
    """Create and return a test debit card."""
    result = run_cli(
        "debit", "add",
        "--name", "Test Debit",
        "--last-four", "9012",
        "--user-id", str(test_user["id"]),
        "--account-id", str(test_account["id"]),
        "--daily-limit", "500"
    )
    assert result["success"] is True
    return result["debit_card"]
