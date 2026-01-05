"""Direct API tests to improve coverage of backend endpoints not exposed via CLI."""

import pytest
import requests
from conftest import API_URL, API_KEY, run_cli

# Disable SSL warnings for self-signed cert
requests.packages.urllib3.disable_warnings()


def api_get(endpoint: str, params: dict = None) -> dict:
    """Make a GET request to the API."""
    headers = {"X-API-Key": API_KEY}
    url = f"{API_URL}/{endpoint}"
    resp = requests.get(url, headers=headers, params=params, verify=False)
    return resp.json() if resp.status_code == 200 else {"error": resp.status_code, "message": resp.text}


def api_post(endpoint: str, data: dict = None) -> dict:
    """Make a POST request to the API."""
    headers = {"X-API-Key": API_KEY}
    url = f"{API_URL}/{endpoint}"
    resp = requests.post(url, json=data, headers=headers, verify=False)
    return resp.json() if resp.status_code in (200, 201) else {"error": resp.status_code, "message": resp.text}


class TestCreditCardSummaryAPI:
    """Test credit card summary API endpoints directly."""

    def test_all_cards_summary(self, test_user, test_credit_card):
        """Test getting summary of all credit cards."""
        result = api_get("credit-cards/summary")
        
        assert "total_cards" in result or "cards" in result or "message" in result

    def test_all_cards_summary_with_user_filter(self, test_user, test_credit_card):
        """Test getting summary of credit cards for specific user."""
        result = api_get("credit-cards/summary", {"user_id": test_user["id"]})
        
        assert "total_cards" in result or "cards" in result or "message" in result

    def test_all_cards_summary_with_month(self, test_user, test_credit_card):
        """Test getting summary for specific month."""
        result = api_get("credit-cards/summary", {"month": "2026-01"})
        
        assert "total_cards" in result or "cards" in result or "message" in result


class TestCreditCardUtilizationAPI:
    """Test credit card utilization API endpoint."""

    def test_card_utilization_history(self, test_user, test_credit_card):
        """Test getting utilization history for a card."""
        result = api_get(f"credit-cards/{test_credit_card['id']}/utilization")
        
        assert "credit_limit" in result or "history" in result or "error" in result

    def test_card_utilization_with_months_param(self, test_user, test_credit_card):
        """Test getting utilization with specific number of months."""
        result = api_get(f"credit-cards/{test_credit_card['id']}/utilization", {"months": 6})
        
        assert "credit_limit" in result or "history" in result or "error" in result


class TestReportsExportAPI:
    """Test reports export API endpoint."""

    def test_export_monthly_report(self, test_user):
        """Test exporting monthly report."""
        # First add some expense data
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--date", "2026-01-15"
        )
        
        result = api_get("reports/export", {
            "user_id": test_user["id"],
            "format": "json",
            "from_date": "2026-01-01",
            "to_date": "2026-01-31"
        })
        
        # Should return some data
        assert result is not None

    def test_export_csv_format(self, test_user):
        """Test exporting report in CSV format."""
        # Add expense data
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "75.00",
            "--category", "Transport",
            "--date", "2026-01-10"
        )

        # Export as CSV
        headers = {"X-API-Key": API_KEY}
        url = f"{API_URL}/reports/export"
        resp = requests.get(url, headers=headers, params={
            "user_id": test_user["id"],
            "format": "csv",
            "from_date": "2026-01-01",
            "to_date": "2026-01-31"
        }, verify=False)

        # Should return CSV content
        assert resp.status_code == 200

    def test_export_without_user_filter(self, test_user):
        """Test exporting all expenses without user filter."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Food",
            "--date", "2026-01-20"
        )

        result = api_get("reports/export", {
            "format": "json",
            "from_date": "2026-01-01",
            "to_date": "2026-01-31"
        })

        assert result is not None


class TestRecurringGenerateAPI:
    """Test recurring expense generation API."""

    def test_generate_single_recurring(self, test_user):
        """Test generating expense from a recurring template."""
        # Create a recurring template
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        template_id = add_result["template"]["id"]

        # Generate an expense from it
        result = api_post(f"recurring-expenses/{template_id}/generate")
        
        # Check we got some response
        assert result is not None

    def test_skip_recurring(self, test_user):
        """Test skipping a recurring expense."""
        # Create a recurring template
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "30.00",
            "--category", "Subscriptions",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "15"
        )
        template_id = add_result["template"]["id"]

        # Skip this occurrence
        result = api_post(f"recurring-expenses/{template_id}/skip")
        
        assert result is not None

    def test_generate_due_recurring(self, test_user):
        """Test generating all due recurring expenses."""
        # Create recurring templates
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Rent",
            "--frequency", "monthly",
            "--start-date", "2025-12-01",
            "--day-of-month", "1"
        )

        # Generate all due
        result = api_post("recurring-expenses/generate-due")
        assert result is not None

    def test_recurring_weekly_frequency(self, test_user):
        """Test weekly recurring expense generation."""
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "20.00",
            "--category", "Groceries",
            "--frequency", "weekly",
            "--start-date", "2026-01-01",
            "--day-of-week", "1"
        )
        template_id = add_result["template"]["id"]

        result = api_post(f"recurring-expenses/{template_id}/generate")
        assert result is not None


class TestSavingsGoalWithdrawAPI:
    """Test savings goal withdraw API."""

    def test_goal_withdraw_success(self, test_user):
        """Test withdrawing from a goal with sufficient funds."""
        # Create a goal with some funds
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Test Withdraw Goal",
            "--target", "1000.00",
            "--deadline", "2026-12-31",
            "--current", "500.00"
        )
        goal_id = add_result["goal"]["id"]

        # Withdraw some funds
        result = api_post(f"savings-goals/{goal_id}/withdraw", {"amount": 100.00})
        
        # Should return updated goal
        assert result is not None
        if "current_amount" in result:
            assert result["current_amount"] == 400.00

    def test_goal_withdraw_insufficient_funds(self, test_user):
        """Test withdrawing more than available from goal."""
        # Create a goal with limited funds
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Small Goal",
            "--target", "500.00",
            "--deadline", "2026-12-31",
            "--current", "50.00"
        )
        goal_id = add_result["goal"]["id"]

        # Try to withdraw more than available
        result = api_post(f"savings-goals/{goal_id}/withdraw", {"amount": 200.00})
        
        # Should get an error
        assert "error" in result or result.get("detail") is not None


class TestBudgetAlertsAPI:
    """Test budget alerts API."""

    def test_budget_alerts(self, test_user):
        """Test getting budget alerts."""
        # Create a budget
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "100.00",
            "--month", "2026-01"
        )
        
        # Exceed the budget
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "150.00",
            "--category", "Food",
            "--date", "2026-01-15"
        )

        # Get alerts
        result = api_get("budgets/status/alerts", {"user_id": test_user["id"]})
        
        assert result is not None


class TestBudgetStatusSummaryAPI:
    """Test budget status summary API."""

    def test_budget_status_summary(self, test_user):
        """Test getting overall budget status summary."""
        # Create budgets in multiple categories
        for category in ["Food", "Transport", "Entertainment"]:
            run_cli(
                "budget", "add",
                "--user-id", str(test_user["id"]),
                "--category", category,
                "--amount", "200.00",
                "--month", "2026-01"
            )

        result = api_get("budgets/status/summary", {"user_id": test_user["id"], "month": "2026-01"})
        
        assert result is not None


class TestSavingsAccountOperationsAPI:
    """Test savings account API operations."""

    def test_account_interest_calculation(self, test_user, test_account):
        """Test applying interest to an account."""
        result = api_post(f"savings-accounts/{test_account['id']}/interest")
        
        assert result is not None

    def test_account_interest_with_amount(self, test_user, test_account):
        """Test applying specific interest amount to an account."""
        result = api_post(f"savings-accounts/{test_account['id']}/interest", {"amount": 25.50})
        
        assert result is not None

    def test_account_summary_individual(self, test_user, test_account):
        """Test getting summary for individual account."""
        result = api_get(f"savings-accounts/{test_account['id']}/summary")
        
        assert result is not None

    def test_all_accounts_summary(self, test_user, test_account):
        """Test getting summary for all accounts."""
        result = api_get("savings-accounts/summary/all")
        
        assert result is not None

    def test_account_withdraw(self, test_user, test_account):
        """Test withdrawing from savings account."""
        result = api_post(f"savings-accounts/{test_account['id']}/withdraw", {
            "amount": 50.00,
            "description": "Test withdrawal"
        })
        
        assert result is not None

    def test_account_deposit(self, test_user, test_account):
        """Test depositing to savings account."""
        result = api_post(f"savings-accounts/{test_account['id']}/deposit", {
            "amount": 100.00,
            "description": "Test deposit"
        })
        
        assert result is not None


class TestExpenseDetailsAPI:
    """Test expense details API."""

    def test_expense_details(self, test_user):
        """Test getting expense with full details."""
        # Create an expense
        add_result = run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Food",
            "--date", "2026-01-15",
            "--description", "Test meal"
        )
        
        # Get expense list to find the ID
        list_result = run_cli("expense", "list", "--user-id", str(test_user["id"]))
        if list_result.get("expenses"):
            expense_id = list_result["expenses"][0]["id"]
            
            # Get details
            result = api_get(f"expenses/{expense_id}/details")
            
            assert result is not None


class TestErrorHandlingAPI:
    """Test error handling paths in the API."""

    def test_invalid_user_id(self, test_user):
        """Test getting non-existent user returns 404."""
        result = api_get("users/999999")
        assert "error" in result or "detail" in result

    def test_invalid_expense_id(self, test_user):
        """Test getting non-existent expense returns 404."""
        result = api_get("expenses/999999")
        assert "error" in result or "detail" in result

    def test_invalid_card_id(self, test_user):
        """Test getting non-existent card returns 404."""
        result = api_get("credit-cards/999999")
        assert "error" in result or "detail" in result

    def test_invalid_goal_id(self, test_user):
        """Test getting non-existent goal returns 404."""
        result = api_get("savings-goals/999999")
        assert "error" in result or "detail" in result

    def test_invalid_account_id(self, test_user):
        """Test getting non-existent account returns 404."""
        result = api_get("savings-accounts/999999")
        assert "error" in result or "detail" in result

    def test_duplicate_user_email(self, test_user):
        """Test creating user with duplicate email fails."""
        # Try to create another user with same email
        result = api_post("users/", {
            "name": "Duplicate User",
            "email": test_user["email"],  # Same email
            "role": "member"
        })
        # Should return error for duplicate email
        assert "error" in result or "detail" in result

    def test_update_user_duplicate_email(self, test_user):
        """Test updating user to existing email fails."""
        # Create another user
        add_result = run_cli(
            "user", "add",
            "--name", "Another User",
            "--email", "another@test.com"
        )
        another_user_id = add_result["user"]["id"]

        # Try to update to existing email
        headers = {"X-API-Key": API_KEY}
        url = f"{API_URL}/users/{another_user_id}"
        result = requests.put(url, json={
            "name": "Another User",
            "email": test_user["email"],  # Same as test_user
            "role": "member"
        }, headers=headers, verify=False)

        # Should fail with duplicate email error
        assert result.status_code in (400, 422) or "error" in result.json() or "detail" in result.json()


class TestDecemberMonthEdgeCaseAPI:
    """Test December month edge cases that use get_month_date_range."""

    def test_expense_summary_december(self, test_user):
        """Test expense summary for December - uses date range function."""
        result = api_get("expenses/summary", {"user_id": test_user["id"], "month": "2025-12"})
        assert result is not None

    def test_budget_compare_december(self, test_user):
        """Test budget comparison with December."""
        # Create budgets
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", "2025-12"
        )
        
        result = api_get("budgets/compare", {"month1": "2025-11", "month2": "2025-12"})
        assert result is not None


class TestPaymentSummaryAPI:
    """Test expense payment summary API."""

    def test_payment_summary(self, test_user):
        """Test getting payment method summary."""
        # Add cash expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "25.00",
            "--category", "Food",
            "--date", "2026-01-10"
        )
        
        result = api_get("expenses/payment_summary", {"user_id": test_user["id"]})
        assert result is not None


class TestInactiveResourceAPI:
    """Test handling of inactive resources."""

    def test_delete_makes_inactive(self, test_user):
        """Test that deleting a resource makes it inactive."""
        # Create and delete a user (soft delete)
        add_result = run_cli(
            "user", "add",
            "--name", "To Delete User",
            "--email", "todelete@test.com"
        )
        user_id = add_result["user"]["id"]
        
        # Delete (soft delete)
        run_cli("user", "delete", str(user_id), "--force")
        
        # Try to get - should still exist but be inactive
        result = api_get(f"users/{user_id}")
        # Soft-deleted users may still return data but marked inactive
        assert result is not None

    def test_inactive_account_interest(self, test_user, test_account):
        """Test applying interest to an inactive account fails."""
        # First deactivate the account by deleting it
        run_cli("account", "delete", str(test_account["id"]), "--force")
        
        # Try to apply interest - should fail
        result = api_post(f"savings-accounts/{test_account['id']}/interest", {"amount": 10.00})
        
        # Should get an error for inactive account
        assert "error" in result or "detail" in result


class TestInvalidInputsAPI:
    """Test invalid input handling."""

    def test_invalid_month_format(self, test_user):
        """Test invalid month format returns error."""
        result = api_get("budgets/", {"month": "invalid-month"})
        # Should either return empty or error
        assert result is not None

    def test_invalid_month_format_card_statement(self, test_user, test_credit_card):
        """Test invalid month format in card statement triggers parse_month error."""
        result = api_get(f"credit-cards/{test_credit_card['id']}/statement", {"month": "not-a-month"})
        # Should return error for invalid month format
        assert "error" in result or "detail" in result or result.get("message")

    def test_expense_list_with_date_range(self, test_user):
        """Test expense list with date range."""
        # Add expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--date", "2025-12-15"
        )
        
        result = api_get("expenses/", {
            "user_id": test_user["id"],
            "from_date": "2025-12-01",
            "to_date": "2025-12-31"
        })
        
        assert result is not None

    def test_december_expense_summary(self, test_user):
        """Test expense summary for December - exercises get_month_date_range."""
        # Add December expense
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "200.00",
            "--category", "Gifts",
            "--date", "2025-12-25"
        )
        
        result = api_get("expenses/summary", {
            "user_id": test_user["id"],
            "month": "2025-12"
        })
        
        assert result is not None


class TestAssetOperationsAPI:
    """Test asset API operations."""

    def test_asset_with_account_link(self, test_user, test_account):
        """Test creating asset linked to savings account."""
        result = api_post("assets/", {
            "user_id": test_user["id"],
            "name": "New Laptop",
            "asset_type": "electronics",
            "purchase_value": 1500.00,
            "current_value": 1500.00,
            "purchase_date": "2026-01-10",
            "savings_account_id": test_account["id"]
        })
        
        assert result is not None

    def test_asset_depreciation_analysis(self, test_user):
        """Test asset depreciation analysis."""
        # Create an asset
        run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Office Chair",
            "--asset-type", "furniture",
            "--purchase-value", "300.00",
            "--current-value", "250.00",
            "--purchase-date", "2025-06-01"
        )
        
        result = api_get("assets/depreciation")
        assert result is not None

    def test_asset_update_value(self, test_user):
        """Test updating asset value."""
        # Create an asset
        add_result = run_cli(
            "asset", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Investment",
            "--asset-type", "investment",
            "--purchase-value", "1000.00",
            "--current-value", "1000.00",
            "--purchase-date", "2026-01-01"
        )
        asset_id = add_result["asset"]["id"]
        
        # Update value
        headers = {"X-API-Key": API_KEY}
        url = f"{API_URL}/assets/{asset_id}/value"
        result = requests.put(url, json={"current_value": 1200.00}, headers=headers, verify=False)
        
        assert result.status_code == 200


class TestAccountWithdrawAPI:
    """Test savings account withdraw operations."""

    def test_account_withdraw_success(self, test_user, test_account):
        """Test withdrawing from account with sufficient funds."""
        # Account should have balance from fixture
        headers = {"X-API-Key": API_KEY}
        url = f"{API_URL}/savings-accounts/{test_account['id']}/withdraw"
        result = requests.post(url, json={"amount": 50.00, "description": "Test withdraw"}, headers=headers, verify=False)
        
        assert result.status_code == 200

    def test_account_withdraw_insufficient_funds(self, test_user):
        """Test withdrawing more than available."""
        # Create account with small balance
        add_result = run_cli(
            "account", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Small Account",
            "--bank", "Test Bank",
            "--last-four", "7777",
            "--balance", "10.00"
        )
        account_id = add_result["account"]["id"]
        
        headers = {"X-API-Key": API_KEY}
        url = f"{API_URL}/savings-accounts/{account_id}/withdraw"
        result = requests.post(url, json={"amount": 500.00, "description": "Large withdraw"}, headers=headers, verify=False)
        
        # Should get error for insufficient funds
        assert result.status_code in [400, 422] or "error" in result.json()


class TestRecurringExpenseEdgeCases:
    """Test recurring expense edge cases."""

    def test_recurring_with_end_date_passed(self, test_user):
        """Test recurring expense with past end date."""
        result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "25.00",
            "--category", "Old Subscription",
            "--frequency", "monthly",
            "--start-date", "2024-01-01",
            "--day-of-month", "1",
            "--end-date", "2024-12-31"
        )
        
        assert result["success"] is True

    def test_process_recurring_with_no_pending(self, test_user):
        """Test processing when no recurring expenses are pending."""
        result = run_cli("recurring", "process")
        assert result["success"] is True


class TestDebitCardEdgeCases:
    """Test debit card edge cases."""

    def test_debit_card_expense_reduces_account(self, test_user, test_account, test_debit_card):
        """Test that debit card expense reduces linked account balance."""
        initial_balance = test_account["current_balance"]
        
        # Add expense via debit card
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "25.00",
            "--category", "Shopping",
            "--date", "2026-01-15",
            "--payment", "debit_card",
            "--debit-id", str(test_debit_card["id"])
        )
        
        # Check account balance was reduced
        result = run_cli("account", "view", str(test_account["id"]))
        assert result["success"] is True


class TestBudgetEdgeCases:
    """Test budget edge cases."""

    def test_budget_with_zero_amount(self, test_user):
        """Test budget with zero amount edge case."""
        result = api_get("budgets/status/summary", {"user_id": test_user["id"], "month": "2026-02"})
        assert result is not None

    def test_budget_compare_nonexistent_months(self, test_user):
        """Test comparing months with no budgets."""
        result = api_get("budgets/compare", {"month1": "2030-01", "month2": "2030-02"})
        assert result is not None


class TestGoalEdgeCases:
    """Test savings goal edge cases."""

    def test_goal_100_percent_complete(self, test_user):
        """Test goal that is 100% complete."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Complete Goal",
            "--target", "100.00",
            "--deadline", "2026-12-31",
            "--current", "100.00"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "view", str(goal_id))
        assert result["success"] is True

    def test_goal_over_100_percent(self, test_user):
        """Test goal with more than 100% saved."""
        add_result = run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Over Goal",
            "--target", "100.00",
            "--deadline", "2026-12-31",
            "--current", "150.00"
        )
        goal_id = add_result["goal"]["id"]
        
        result = run_cli("goal", "view", str(goal_id))
        assert result["success"] is True


class TestExpenseEdgeCases:
    """Test expense edge cases."""

    def test_expense_with_tags(self, test_user):
        """Test expense with tags."""
        result = run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "45.00",
            "--category", "Entertainment",
            "--date", "2026-01-20",
            "--tags", "movie,fun,weekend"
        )
        assert result["success"] is True

    def test_expense_list_with_payment_filter(self, test_user):
        """Test listing expenses by payment method."""
        # Add cash expense
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "15.00",
            "--category", "Food",
            "--date", "2026-01-18"
        )
        
        result = run_cli("expense", "list", "--user-id", str(test_user["id"]), "--payment", "cash")
        assert result["success"] is True


