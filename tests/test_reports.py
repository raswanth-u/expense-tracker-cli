"""Test report generation operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestReportMonthly:
    """Test monthly report command."""
    
    def test_monthly_report_empty(self, test_user):
        """Test monthly report with no expenses."""
        result = run_cli("report", "monthly", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["month"] == "2026-01"
        assert "report" in result
        # When empty, report has total_spent directly, not in summary
        assert result["report"]["total_spent"] == 0 or result["report"].get("summary", {}).get("total_spent", 0) == 0
    
    def test_monthly_report_with_expenses(self, test_user):
        """Test monthly report with expenses."""
        # Create expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Food",
            "--date", "2026-01-01"
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--date", "2026-01-05"
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "200.00",
            "--category", "Housing",
            "--date", "2026-01-10"
        )
        
        result = run_cli("report", "monthly", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["report"]["summary"]["total_spent"] == 350.0
        assert result["report"]["summary"]["transaction_count"] == 3
    
    def test_monthly_report_by_category(self, test_user):
        """Test monthly report category breakdown."""
        # Create expenses
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "100.00", "--category", "Food", "--date", "2026-01-01")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "50.00", "--category", "Food", "--date", "2026-01-02")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "200.00", "--category", "Housing", "--date", "2026-01-03")
        
        result = run_cli("report", "monthly", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "by_category" in result["report"]
        assert result["report"]["by_category"]["Food"] == 150.0
        assert result["report"]["by_category"]["Housing"] == 200.0
    
    def test_monthly_report_top_expenses(self, test_user):
        """Test monthly report shows top expenses."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "500.00", "--category", "Big", "--date", "2026-01-01")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "50.00", "--category", "Small", "--date", "2026-01-02")
        
        result = run_cli("report", "monthly", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert len(result["report"]["top_expenses"]) >= 1
        assert result["report"]["top_expenses"][0]["amount"] == 500.0


class TestReportCategory:
    """Test category analysis report command."""
    
    def test_category_report(self, test_user):
        """Test category analysis report."""
        # Create expenses in category
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "25.00", "--category", "Groceries", "--date", "2026-01-05")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "35.00", "--category", "Groceries", "--date", "2026-01-15")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "45.00", "--category", "Groceries", "--date", "2026-01-25")
        
        result = run_cli(
            "report", "category", "Groceries",
            "--from", "2026-01-01",
            "--to", "2026-01-31",
            "--user-id", str(test_user["id"])
        )
        
        assert result["success"] is True
        assert result["category"] == "Groceries"
        assert "analysis" in result
        assert result["analysis"]["summary"]["total_spent"] == 105.0
        assert result["analysis"]["summary"]["transaction_count"] == 3
    
    def test_category_report_averages(self, test_user):
        """Test category report shows averages correctly."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "100.00", "--category", "Entertainment", "--date", "2026-01-01")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "200.00", "--category", "Entertainment", "--date", "2026-01-15")
        
        result = run_cli(
            "report", "category", "Entertainment",
            "--from", "2026-01-01",
            "--to", "2026-01-31",
            "--user-id", str(test_user["id"])
        )
        
        assert result["success"] is True
        assert result["analysis"]["summary"]["average_transaction"] == 150.0
        assert result["analysis"]["summary"]["highest_transaction"] == 200.0
        assert result["analysis"]["summary"]["lowest_transaction"] == 100.0


class TestReportFamily:
    """Test family summary report command."""
    
    def test_family_report(self, test_user):
        """Test family summary report."""
        # Create expenses for user
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "100.00", "--category", "Food", "--date", "2026-01-01")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "200.00", "--category", "Housing", "--date", "2026-01-15")
        
        result = run_cli("report", "family", "--month", "2026-01")
        
        assert result["success"] is True
        assert result["month"] == "2026-01"
        assert "summary" in result
        assert result["summary"]["family_total"] >= 300.0
    
    def test_family_report_member_breakdown(self, test_user):
        """Test family report shows member breakdown."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "500.00", "--category", "Rent", "--date", "2026-01-01")
        
        result = run_cli("report", "family", "--month", "2026-01")
        
        assert result["success"] is True
        assert "members" in result["summary"]
        
        # Find our test user in the members
        user_data = None
        for member in result["summary"]["members"]:
            if member["user_id"] == test_user["id"]:
                user_data = member
                break
        
        if user_data:
            assert user_data["total_spent"] >= 500.0


class TestReportTrends:
    """Test spending trends report command."""
    
    def test_trends_report(self, test_user):
        """Test spending trends report."""
        # Create expenses in different months
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "100.00", "--category", "Food", "--date", "2025-12-15")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "150.00", "--category", "Food", "--date", "2026-01-15")
        
        result = run_cli("report", "trends", "--months", "3", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["months"] == 3
        assert "trends" in result
        assert result["trends"]["months_analyzed"] == 3
    
    def test_trends_report_monthly_data(self, test_user):
        """Test trends report shows monthly data."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "200.00", "--category", "Housing", "--date", "2026-01-01")
        
        result = run_cli("report", "trends", "--months", "6", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "monthly_data" in result["trends"]
        assert len(result["trends"]["monthly_data"]) == 6


class TestReportPayments:
    """Test payment method analysis report command."""
    
    def test_payments_report(self, test_user):
        """Test payment method analysis report."""
        # Create cash expenses
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "50.00", "--category", "Food", "--date", "2026-01-01")
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "75.00", "--category", "Food", "--date", "2026-01-02")
        
        result = run_cli("report", "payments", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["month"] == "2026-01"
        assert "analysis" in result
        assert result["analysis"]["total_spent"] == 125.0
    
    def test_payments_report_breakdown(self, test_user):
        """Test payment report shows method breakdown."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "100.00", "--category", "Shopping", "--date", "2026-01-05")
        
        result = run_cli("report", "payments", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "by_payment_method" in result["analysis"]
        
        # Should have at least cash method
        methods = result["analysis"]["by_payment_method"]
        assert len(methods) >= 1
        
        # Find cash method
        cash_method = next((m for m in methods if m["payment_method"] == "cash"), None)
        if cash_method:
            assert cash_method["total_spent"] >= 100.0


class TestReportDisplay:
    """Test report display output (non-JSON)."""
    
    def test_monthly_display(self, test_user):
        """Test that monthly report displays without errors."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "100.00", "--category", "Display", "--date", "2026-01-01")
        
        returncode, stdout, stderr = run_cli_display("report", "monthly", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
    
    def test_category_display(self, test_user):
        """Test that category report displays without errors."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "50.00", "--category", "TestCat", "--date", "2026-01-01")
        
        returncode, stdout, stderr = run_cli_display(
            "report", "category", "TestCat",
            "--from", "2026-01-01",
            "--to", "2026-01-31",
            "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0
    
    def test_family_display(self, test_user):
        """Test that family report displays without errors."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "150.00", "--category", "Family", "--date", "2026-01-01")
        
        returncode, stdout, stderr = run_cli_display("report", "family", "--month", "2026-01")
        
        assert returncode == 0
    
    def test_trends_display(self, test_user):
        """Test that trends report displays without errors."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "200.00", "--category", "Trends", "--date", "2026-01-01")
        
        returncode, stdout, stderr = run_cli_display("report", "trends", "--months", "3", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
    
    def test_payments_display(self, test_user):
        """Test that payments report displays without errors."""
        run_cli("expense", "add", "--user-id", str(test_user["id"]), "--amount", "75.00", "--category", "Payments", "--date", "2026-01-01")
        
        returncode, stdout, stderr = run_cli_display("report", "payments", "--month", "2026-01", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
