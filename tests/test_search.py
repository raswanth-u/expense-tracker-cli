"""Test search operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestSearchExpenses:
    """Test expense search command."""
    
    def test_search_by_description(self, test_user):
        """Test searching expenses by description."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Groceries",
            "--description", "Weekly grocery shopping",
            "--date", "2026-01-01"
        )
        
        result = run_cli("search", "grocery")
        
        assert result["success"] is True
        assert result["count"] >= 1
        assert result["query"] == "grocery"
        
        # Verify result contains our expense
        found = any(
            e.get("description") == "Weekly grocery shopping"
            for e in result.get("results", [])
        )
        assert found
    
    def test_search_by_category(self, test_user):
        """Test searching expenses by category."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Entertainment",
            "--description", "Movie tickets",
            "--date", "2026-01-05"
        )
        
        result = run_cli("search", "Entertainment")
        
        assert result["success"] is True
        assert result["count"] >= 1
    
    def test_search_no_results(self, test_user):
        """Test search with no matching results."""
        result = run_cli("search", "nonexistentterm12345")
        
        assert result["success"] is True
        assert result["count"] == 0


class TestSearchEntities:
    """Test search by entity type."""
    
    def test_search_users(self, test_user):
        """Test searching users."""
        result = run_cli("search", "test", "--entity", "users")
        
        assert result["success"] is True
        assert result["entity"] == "users"
        # Should find our test user
        assert result["count"] >= 1
    
    def test_search_expenses_entity(self, test_user):
        """Test searching expenses explicitly."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "75.00",
            "--category", "Dining",
            "--description", "Restaurant dinner",
            "--date", "2026-01-10"
        )
        
        result = run_cli("search", "Restaurant", "--entity", "expenses")
        
        assert result["success"] is True
        assert result["entity"] == "expenses"
        assert result["count"] >= 1


class TestSearchFilters:
    """Test search with filters."""
    
    def test_search_date_range(self, test_user):
        """Test search with date range filter."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--description", "Date range test",
            "--date", "2026-01-15"
        )
        
        result = run_cli(
            "search", "Food",
            "--from", "2026-01-01",
            "--to", "2026-01-31"
        )
        
        assert result["success"] is True
        # Should find expenses in date range
        for expense in result.get("results", []):
            if "date" in expense:
                assert expense["date"] >= "2026-01-01"
                assert expense["date"] <= "2026-01-31"
    
    def test_search_amount_range(self, test_user):
        """Test search with amount range filter."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "150.00",
            "--category", "Shopping",
            "--description", "Amount filter test",
            "--date", "2026-01-20"
        )
        
        result = run_cli(
            "search", "Shopping",
            "--min", "100",
            "--max", "200"
        )
        
        assert result["success"] is True
        # All results should be within amount range
        for expense in result.get("results", []):
            if "amount" in expense:
                assert expense["amount"] >= 100
                assert expense["amount"] <= 200


class TestSearchSummary:
    """Test search summary calculations."""
    
    def test_search_summary_total(self, test_user):
        """Test search summary includes total."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "SummaryTest",
            "--date", "2026-01-01"
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "75.00",
            "--category", "SummaryTest",
            "--date", "2026-01-02"
        )
        
        result = run_cli("search", "SummaryTest")
        
        assert result["success"] is True
        if "summary" in result:
            assert result["summary"]["count"] >= 2
            assert result["summary"]["total"] >= 125.0


class TestSearchDisplay:
    """Test search display output (non-JSON)."""
    
    def test_search_display(self, test_user):
        """Test that search displays without errors."""
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "DisplaySearch",
            "--description", "Display test expense",
            "--date", "2026-01-01"
        )
        
        returncode, stdout, stderr = run_cli_display("search", "Display")
        
        assert returncode == 0
    
    def test_search_users_display(self, test_user):
        """Test that user search displays without errors."""
        returncode, stdout, stderr = run_cli_display("search", "test", "--entity", "users")
        
        assert returncode == 0
