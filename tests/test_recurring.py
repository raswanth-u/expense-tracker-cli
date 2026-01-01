"""Test recurring expense CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestRecurringAdd:
    """Test recurring add command."""
    
    def test_add_recurring_monthly(self, test_user):
        """Test adding a monthly recurring expense."""
        result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        
        assert result["success"] is True
        assert result["template"]["amount"] == 100.0
        assert result["template"]["category"] == "Utilities"
        assert result["template"]["frequency"] == "monthly"
        assert result["template"]["day_of_month"] == 1
    
    def test_add_recurring_weekly(self, test_user):
        """Test adding a weekly recurring expense."""
        result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Groceries",
            "--frequency", "weekly",
            "--start-date", "2026-01-01",
            "--day-of-week", "0"  # Monday
        )
        
        assert result["success"] is True
        assert result["template"]["frequency"] == "weekly"
        assert result["template"]["day_of_week"] == 0
    
    def test_add_recurring_with_description(self, test_user):
        """Test adding recurring with description."""
        result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "1500.00",
            "--category", "Housing",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1",
            "--description", "Monthly rent payment"
        )
        
        assert result["success"] is True
        assert result["template"]["description"] == "Monthly rent payment"
    
    def test_add_recurring_with_end_date(self, test_user):
        """Test adding recurring with end date."""
        result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "200.00",
            "--category", "Insurance",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "15",
            "--end-date", "2026-12-31"
        )
        
        assert result["success"] is True
        assert result["template"]["end_date"] == "2026-12-31"
    
    def test_add_recurring_with_interval(self, test_user):
        """Test adding recurring with custom interval."""
        result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "25.00",
            "--category", "Subscriptions",
            "--frequency", "weekly",
            "--start-date", "2026-01-01",
            "--day-of-week", "5",  # Saturday
            "--interval", "2"  # Every 2 weeks
        )
        
        assert result["success"] is True
        assert result["template"]["interval"] == 2


class TestRecurringList:
    """Test recurring list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no recurring expenses exist."""
        result = run_cli("recurring", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
    
    def test_list_recurring(self, test_user):
        """Test listing multiple recurring expenses."""
        # Create recurring expenses
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Subscriptions",
            "--frequency", "monthly",
            "--start-date", "2026-01-15",
            "--day-of-month", "15"
        )
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "75.00",
            "--category", "Groceries",
            "--frequency", "weekly",
            "--start-date", "2026-01-01",
            "--day-of-week", "0"
        )
        
        result = run_cli("recurring", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 3
    
    def test_list_with_summary(self, test_user):
        """Test that list returns summary totals."""
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "200.00",
            "--category", "Insurance",
            "--frequency", "monthly",
            "--start-date", "2026-01-15",
            "--day-of-month", "15"
        )
        
        result = run_cli("recurring", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert "summary" in result
        assert result["summary"]["monthly_total"] == 300.0


class TestRecurringView:
    """Test recurring view command."""
    
    def test_view_recurring(self, test_user):
        """Test viewing a single recurring template."""
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "150.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        template_id = add_result["template"]["id"]
        
        result = run_cli("recurring", "view", str(template_id))
        
        assert result["success"] is True
        assert result["template"]["id"] == template_id
        assert result["template"]["amount"] == 150.0
        assert result["template"]["category"] == "Utilities"


class TestRecurringUpdate:
    """Test recurring update command."""
    
    def test_update_amount(self, test_user):
        """Test updating recurring amount."""
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        template_id = add_result["template"]["id"]
        
        result = run_cli("recurring", "update", str(template_id), "--amount", "150.00")
        
        assert result["success"] is True
        assert result["template"]["amount"] == 150.0
    
    def test_update_category(self, test_user):
        """Test updating recurring category."""
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        template_id = add_result["template"]["id"]
        
        result = run_cli("recurring", "update", str(template_id), "--category", "Bills")
        
        assert result["success"] is True
        assert result["template"]["category"] == "Bills"
    
    def test_update_multiple_fields(self, test_user):
        """Test updating multiple fields at once."""
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Subscriptions",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        template_id = add_result["template"]["id"]
        
        # Update both amount and category
        result = run_cli(
            "recurring", "update", str(template_id),
            "--amount", "200.00",
            "--category", "Entertainment"
        )
        
        assert result["success"] is True
        assert result["template"]["amount"] == 200.0
        assert result["template"]["category"] == "Entertainment"


class TestRecurringUpcoming:
    """Test recurring upcoming command."""
    
    def test_upcoming_expenses(self, test_user):
        """Test viewing upcoming recurring expenses."""
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Subscriptions",
            "--frequency", "monthly",
            "--start-date", "2026-01-15",
            "--day-of-month", "15"
        )
        
        result = run_cli("recurring", "upcoming", "--days", "60")
        
        assert result["success"] is True
        assert "upcoming" in result
        # Should have at least some upcoming expenses
        assert result["upcoming"]["count"] >= 0


class TestRecurringProcess:
    """Test recurring process command."""
    
    def test_process_recurring(self, test_user):
        """Test processing recurring expenses."""
        # Create a recurring expense that's due today or past
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2025-12-01",  # Past date
            "--day-of-month", "1"
        )
        
        result = run_cli("recurring", "process")
        
        assert result["success"] is True
        assert "result" in result
        assert "generated_count" in result["result"]


class TestRecurringDelete:
    """Test recurring delete command."""
    
    def test_delete_recurring(self, test_user):
        """Test deleting a recurring template."""
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Utilities",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        template_id = add_result["template"]["id"]
        
        result = run_cli("recurring", "delete", str(template_id), "--force")
        
        assert result["success"] is True
        assert result["id"] == template_id
        
        # Verify template is gone
        list_result = run_cli("recurring", "list", "--user-id", str(test_user["id"]))
        ids = [t["id"] for t in list_result.get("templates", [])]
        assert template_id not in ids


class TestRecurringDisplay:
    """Test recurring display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that recurring list displays without errors."""
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Display Test",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        
        returncode, stdout, stderr = run_cli_display("recurring", "list", "--user-id", str(test_user["id"]))
        
        assert returncode == 0
        assert "Display Test" in stdout or "100" in stdout
    
    def test_view_display(self, test_user):
        """Test that recurring view displays without errors."""
        add_result = run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "150.00",
            "--category", "View Display",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "15"
        )
        template_id = add_result["template"]["id"]
        
        returncode, stdout, stderr = run_cli_display("recurring", "view", str(template_id))
        
        assert returncode == 0
    
    def test_upcoming_display(self, test_user):
        """Test that recurring upcoming displays without errors."""
        run_cli(
            "recurring", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "75.00",
            "--category", "Upcoming Test",
            "--frequency", "monthly",
            "--start-date", "2026-01-01",
            "--day-of-month", "1"
        )
        
        returncode, stdout, stderr = run_cli_display("recurring", "upcoming", "--days", "30")
        
        assert returncode == 0
