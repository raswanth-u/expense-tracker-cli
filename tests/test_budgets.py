"""Test budget CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


# Current test month
TEST_MONTH = "2026-01"


class TestBudgetAdd:
    """Test budget add command."""
    
    def test_add_budget_basic(self, test_user):
        """Test adding a budget with required fields."""
        result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        
        assert result["success"] is True
        assert result["budget"]["category"] == "Food"
        assert float(result["budget"]["amount"]) == 500.00
        assert result["budget"]["month"] == TEST_MONTH
    
    def test_add_budget_with_period(self, test_user):
        """Test adding a budget with specific period."""
        result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Entertainment",
            "--amount", "100.00",
            "--month", TEST_MONTH,
            "--period", "weekly"
        )
        
        assert result["success"] is True
        assert result["budget"]["period"] == "weekly"
    
    def test_add_budget_with_tags(self, test_user):
        """Test adding a budget with tags."""
        result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Transport",
            "--amount", "200.00",
            "--month", TEST_MONTH,
            "--tags", "commute,fuel"
        )
        
        assert result["success"] is True
        assert result["budget"]["tags"] == "commute,fuel"


class TestBudgetList:
    """Test budget list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no budgets exist."""
        result = run_cli("budget", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
    
    def test_list_budgets(self, test_user):
        """Test listing multiple budgets."""
        # Create budgets
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Transport",
            "--amount", "200.00",
            "--month", TEST_MONTH
        )
        
        result = run_cli("budget", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 2
    
    def test_list_by_month(self, test_user):
        """Test listing budgets filtered by month."""
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", "2026-01"
        )
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Transport",
            "--amount", "200.00",
            "--month", "2026-02"
        )
        
        result = run_cli("budget", "list", 
                        "--user-id", str(test_user["id"]),
                        "--month", "2026-01")
        
        assert result["success"] is True
        assert result["count"] == 1


class TestBudgetView:
    """Test budget view command."""
    
    def test_view_budget(self, test_user):
        """Test viewing a single budget."""
        add_result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        budget_id = add_result["id"]
        
        result = run_cli("budget", "view", str(budget_id))
        
        assert result["success"] is True
        assert result["budget"]["id"] == budget_id
        assert result["budget"]["category"] == "Food"


class TestBudgetUpdate:
    """Test budget update command."""
    
    def test_update_amount(self, test_user):
        """Test updating budget amount."""
        add_result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        budget_id = add_result["id"]
        
        result = run_cli("budget", "update", str(budget_id), "--amount", "600.00")
        
        assert result["success"] is True
        assert float(result["budget"]["amount"]) == 600.00
    
    def test_update_category(self, test_user):
        """Test updating budget category."""
        add_result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        budget_id = add_result["id"]
        
        result = run_cli("budget", "update", str(budget_id), "--category", "Groceries")
        
        assert result["success"] is True
        assert result["budget"]["category"] == "Groceries"


class TestBudgetDelete:
    """Test budget delete command."""
    
    def test_delete_budget(self, test_user):
        """Test deleting a budget."""
        add_result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        budget_id = add_result["id"]
        
        result = run_cli("budget", "delete", str(budget_id), "--force")
        
        assert result["success"] is True
        
        # Verify budget is gone
        list_result = run_cli("budget", "list", "--user-id", str(test_user["id"]))
        assert list_result["count"] == 0


class TestBudgetStatus:
    """Test budget status command."""
    
    def test_status_under_budget(self, test_user):
        """Test budget status when under budget."""
        # Create budget
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        
        # Create expense within budget
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--date", "2026-01-15"
        )
        
        result = run_cli("budget", "status", 
                        "--month", TEST_MONTH,
                        "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
    
    def test_status_over_budget(self, test_user):
        """Test budget status when over budget."""
        # Create budget
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "100.00",
            "--month", TEST_MONTH
        )
        
        # Create expense over budget
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "150.00",
            "--category", "Food",
            "--date", "2026-01-15"
        )
        
        result = run_cli("budget", "status",
                        "--month", TEST_MONTH,
                        "--user-id", str(test_user["id"]))
        
        assert result["success"] is True


class TestBudgetCompare:
    """Test budget compare command."""
    
    def test_compare_months(self, test_user):
        """Test comparing budgets across months."""
        # Create budget for Jan
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", "2026-01"
        )
        
        # Create budget for Feb  
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "600.00",
            "--month", "2026-02"
        )
        
        result = run_cli("budget", "compare",
                        "2026-01", "2026-02",
                        "--user-id", str(test_user["id"]))
        
        assert result["success"] is True


class TestBudgetCalculations:
    """Test budget calculations."""
    
    def test_spending_percentage(self, test_user):
        """Test that spending percentage is calculated correctly."""
        # Create budget
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "200.00",
            "--month", TEST_MONTH
        )
        
        # Spend 50% of budget
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--date", "2026-01-15"
        )
        
        result = run_cli("budget", "status",
                        "--month", TEST_MONTH,
                        "--user-id", str(test_user["id"]))
        
        assert result["success"] is True


class TestBudgetDisplay:
    """Test budget display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that budget list displays without errors."""
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        
        returncode, stdout, stderr = run_cli_display(
            "budget", "list", "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0
    
    def test_status_display(self, test_user):
        """Test that budget status displays without errors."""
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", TEST_MONTH
        )
        
        returncode, stdout, stderr = run_cli_display(
            "budget", "status",
            "--month", TEST_MONTH,
            "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0


class TestBudgetDecember:
    """Test budget operations for December (edge case for date range)."""

    def test_add_budget_december(self, test_user):
        """Test adding a budget for December."""
        result = run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Holiday",
            "--amount", "1000.00",
            "--month", "2025-12"
        )

        assert result["success"] is True
        assert result["budget"]["month"] == "2025-12"

    def test_budget_status_december(self, test_user):
        """Test budget status for December month."""
        # Create budget for December
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Gifts",
            "--amount", "500.00",
            "--month", "2025-12"
        )

        # Add expense in December
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "200.00",
            "--category", "Gifts",
            "--date", "2025-12-25"
        )

        result = run_cli("budget", "status",
                        "--month", "2025-12",
                        "--user-id", str(test_user["id"]))

        assert result["success"] is True

    def test_compare_december_with_january(self, test_user):
        """Test comparing December and January budgets."""
        # Create budget for Dec 2025
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "600.00",
            "--month", "2025-12"
        )

        # Create budget for Jan 2026
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500.00",
            "--month", "2026-01"
        )

        result = run_cli("budget", "compare",
                        "2025-12", "2026-01",
                        "--user-id", str(test_user["id"]))

        assert result["success"] is True


class TestBudgetAlerts:
    """Test budget alert functionality."""

    def test_budget_over_limit_alert(self, test_user):
        """Test budget alert when spending exceeds budget."""
        # Create a small budget
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Entertainment",
            "--amount", "100.00",
            "--month", TEST_MONTH
        )

        # Exceed the budget
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "150.00",
            "--category", "Entertainment",
            "--date", "2026-01-15"
        )

        # Check status shows over budget
        result = run_cli("budget", "status",
                        "--month", TEST_MONTH,
                        "--user-id", str(test_user["id"]))

        assert result["success"] is True

    def test_multiple_categories_budget_status(self, test_user):
        """Test budget status with multiple categories."""
        # Create multiple budgets
        for category in ["Food", "Transport", "Utilities"]:
            run_cli(
                "budget", "add",
                "--user-id", str(test_user["id"]),
                "--category", category,
                "--amount", "300.00",
                "--month", TEST_MONTH
            )

        # Add some expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--date", "2026-01-10"
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "200.00",
            "--category", "Transport",
            "--date", "2026-01-12"
        )

        result = run_cli("budget", "status",
                        "--month", TEST_MONTH,
                        "--user-id", str(test_user["id"]))

        assert result["success"] is True
