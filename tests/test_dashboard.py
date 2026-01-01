"""Test dashboard command."""

import pytest
from conftest import run_cli, run_cli_display


class TestDashboardBasic:
    """Test basic dashboard command."""
    
    def test_dashboard_empty(self, test_user):
        """Test dashboard with no data."""
        result = run_cli("dashboard", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
    
    def test_dashboard_with_month(self, test_user):
        """Test dashboard for specific month."""
        result = run_cli("dashboard", "--user-id", str(test_user["id"]),
                        "--month", "2026-01")
        
        assert result["success"] is True


class TestDashboardWithData:
    """Test dashboard with data."""
    
    def test_dashboard_with_expenses(self, test_user):
        """Test dashboard shows expense data."""
        # Create expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100",
            "--category", "Food",
            "--date", "2026-01-15"
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50",
            "--category", "Transport",
            "--date", "2026-01-16"
        )
        
        result = run_cli("dashboard", "--user-id", str(test_user["id"]),
                        "--month", "2026-01")
        
        assert result["success"] is True
    
    def test_dashboard_with_budgets(self, test_user):
        """Test dashboard shows budget data."""
        # Create budget
        run_cli(
            "budget", "add",
            "--user-id", str(test_user["id"]),
            "--category", "Food",
            "--amount", "500",
            "--month", "2026-01"
        )
        
        result = run_cli("dashboard", "--user-id", str(test_user["id"]),
                        "--month", "2026-01")
        
        assert result["success"] is True
    
    def test_dashboard_with_savings(self, test_user, test_account):
        """Test dashboard shows savings data."""
        result = run_cli("dashboard", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
    
    def test_dashboard_with_credit_cards(self, test_user, test_credit_card):
        """Test dashboard shows credit card data."""
        result = run_cli("dashboard", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
    
    def test_dashboard_with_goals(self, test_user):
        """Test dashboard shows savings goal data."""
        run_cli(
            "goal", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Test Goal",
            "--target", "1000",
            "--deadline", "2026-12-31"
        )
        
        result = run_cli("dashboard", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True


class TestDashboardDisplay:
    """Test dashboard display output (non-JSON)."""
    
    def test_dashboard_display_empty(self, test_user):
        """Test dashboard display with no data."""
        returncode, stdout, stderr = run_cli_display(
            "dashboard", "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0
    
    def test_dashboard_display_with_data(self, test_user):
        """Test dashboard display with data."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100",
            "--category", "Food"
        )
        
        returncode, stdout, stderr = run_cli_display(
            "dashboard", "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0
