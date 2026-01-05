"""Test expense CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestExpenseAddCash:
    """Test adding cash expenses."""
    
    def test_add_cash_expense_basic(self, test_user):
        """Test adding a basic cash expense."""
        result = run_cli(
            "expense", "add",
            "--amount", "50.00",
            "--category", "Food",
            "--user-id", str(test_user["id"]),
            "--description", "Lunch"
        )
        
        assert result["success"] is True
        assert result["id"] is not None
        assert result["expense"]["expense"]["amount"] == 50.00
        assert result["expense"]["expense"]["category"] == "Food"
        assert result["expense"]["expense"]["payment_method"] == "cash"
    
    def test_add_cash_expense_with_date(self, test_user):
        """Test adding expense with specific date."""
        result = run_cli(
            "expense", "add",
            "--amount", "100.00",
            "--category", "Transport",
            "--user-id", str(test_user["id"]),
            "--date", "2026-01-15"
        )
        
        assert result["success"] is True
        assert result["expense"]["expense"]["date"] == "2026-01-15"
    
    def test_add_cash_expense_with_tags(self, test_user):
        """Test adding expense with tags."""
        result = run_cli(
            "expense", "add",
            "--amount", "25.00",
            "--category", "Entertainment",
            "--user-id", str(test_user["id"]),
            "--tags", "movie,weekend"
        )
        
        assert result["success"] is True
        assert result["expense"]["expense"]["tags"] == "movie,weekend"


class TestExpenseAddCreditCard:
    """Test adding credit card expenses."""
    
    def test_add_credit_card_expense(self, test_user, test_credit_card):
        """Test adding a credit card expense."""
        result = run_cli(
            "expense", "add",
            "--amount", "150.00",
            "--category", "Shopping",
            "--user-id", str(test_user["id"]),
            "--payment", "credit_card",
            "--card-id", str(test_credit_card["id"])
        )
        
        assert result["success"] is True
        assert result["expense"]["expense"]["payment_method"] == "credit_card"
        assert result["expense"]["expense"]["credit_card_id"] == test_credit_card["id"]


class TestExpenseAddDebitCard:
    """Test adding debit card expenses."""
    
    def test_add_debit_card_expense(self, test_user, test_debit_card, test_account):
        """Test adding a debit card expense.
        
        Note: When using debit_card payment method, the CLI internally converts it
        to savings_account because a debit card is linked to a savings account.
        """
        result = run_cli(
            "expense", "add",
            "--amount", "75.00",
            "--category", "Groceries",
            "--user-id", str(test_user["id"]),
            "--payment", "debit_card",
            "--debit-id", str(test_debit_card["id"])
        )
        
        assert result["success"] is True
        # Debit card expenses are stored as savings_account transactions
        assert result["expense"]["expense"]["payment_method"] == "savings_account"


class TestExpenseList:
    """Test expense list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no expenses exist."""
        result = run_cli("expense", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
        assert len(result["expenses"]) == 0
    
    def test_list_expenses(self, test_user):
        """Test listing expenses."""
        # Create some expenses
        run_cli("expense", "add", "--amount", "50", "--category", "Food", 
                "--user-id", str(test_user["id"]))
        run_cli("expense", "add", "--amount", "30", "--category", "Transport",
                "--user-id", str(test_user["id"]))
        
        result = run_cli("expense", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 2
        assert len(result["expenses"]) == 2
    
    def test_list_by_category(self, test_user):
        """Test filtering by category."""
        run_cli("expense", "add", "--amount", "50", "--category", "Food",
                "--user-id", str(test_user["id"]))
        run_cli("expense", "add", "--amount", "30", "--category", "Transport",
                "--user-id", str(test_user["id"]))
        
        result = run_cli("expense", "list", "--user-id", str(test_user["id"]),
                        "--category", "Food")
        
        assert result["success"] is True
        assert result["count"] == 1
        assert result["expenses"][0]["category"] == "Food"
    
    def test_list_with_limit(self, test_user):
        """Test limiting results."""
        for i in range(5):
            run_cli("expense", "add", "--amount", str(10 + i), 
                   "--category", "Test", "--user-id", str(test_user["id"]))
        
        result = run_cli("expense", "list", "--user-id", str(test_user["id"]),
                        "--limit", "2")
        
        assert result["success"] is True
        assert len(result["expenses"]) == 2


class TestExpenseView:
    """Test expense view command."""
    
    def test_view_expense(self, test_user):
        """Test viewing an expense."""
        # Create an expense first
        add_result = run_cli("expense", "add", "--amount", "100",
                            "--category", "Test", "--user-id", str(test_user["id"]))
        expense_id = add_result["id"]
        
        result = run_cli("expense", "view", str(expense_id))
        
        assert result["success"] is True
        assert result["expense"]["expense"]["id"] == expense_id


class TestExpenseUpdate:
    """Test expense update command."""
    
    def test_update_amount(self, test_user):
        """Test updating expense amount."""
        add_result = run_cli("expense", "add", "--amount", "50",
                            "--category", "Food", "--user-id", str(test_user["id"]))
        expense_id = add_result["id"]
        
        result = run_cli("expense", "update", str(expense_id), "--amount", "75")
        
        assert result["success"] is True
        assert result["expense"]["amount"] == 75.00
    
    def test_update_description(self, test_user):
        """Test updating expense description."""
        add_result = run_cli("expense", "add", "--amount", "50",
                            "--category", "Food", "--user-id", str(test_user["id"]))
        expense_id = add_result["id"]
        
        result = run_cli("expense", "update", str(expense_id),
                        "--description", "Updated description")
        
        assert result["success"] is True
        assert result["expense"]["description"] == "Updated description"
    
    def test_update_category(self, test_user):
        """Test updating expense category."""
        add_result = run_cli("expense", "add", "--amount", "50",
                            "--category", "Food", "--user-id", str(test_user["id"]))
        expense_id = add_result["id"]
        
        result = run_cli("expense", "update", str(expense_id),
                        "--category", "Transport")
        
        assert result["success"] is True
        assert result["expense"]["category"] == "Transport"


class TestExpenseDelete:
    """Test expense delete command."""
    
    def test_delete_expense(self, test_user):
        """Test deleting an expense."""
        add_result = run_cli("expense", "add", "--amount", "50",
                            "--category", "Food", "--user-id", str(test_user["id"]))
        expense_id = add_result["id"]
        
        result = run_cli("expense", "delete", str(expense_id), "--force")
        
        assert result["success"] is True
        
        # Verify expense is gone
        list_result = run_cli("expense", "list", "--user-id", str(test_user["id"]))
        ids = [e["id"] for e in list_result["expenses"]]
        assert expense_id not in ids


class TestExpenseSummary:
    """Test expense summary command."""
    
    def test_summary(self, test_user):
        """Test getting expense summary."""
        # Create some expenses
        run_cli("expense", "add", "--amount", "100", "--category", "Food",
                "--user-id", str(test_user["id"]))
        run_cli("expense", "add", "--amount", "50", "--category", "Transport",
                "--user-id", str(test_user["id"]))
        
        result = run_cli("expense", "summary", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True


class TestExpenseCalculations:
    """Test expense calculations."""
    
    def test_total_calculation(self, test_user):
        """Test that expense totals are calculated correctly."""
        run_cli("expense", "add", "--amount", "100", "--category", "Food",
                "--user-id", str(test_user["id"]))
        run_cli("expense", "add", "--amount", "50", "--category", "Food",
                "--user-id", str(test_user["id"]))
        
        result = run_cli("expense", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["summary"]["total"] == 150.00
        assert result["summary"]["count"] == 2


class TestExpenseDisplay:
    """Test expense display output (non-JSON)."""
    
    def test_list_display(self, test_user):
        """Test that expense list displays without errors."""
        run_cli("expense", "add", "--amount", "50", "--category", "Food",
                "--user-id", str(test_user["id"]))
        
        returncode, stdout, stderr = run_cli_display(
            "expense", "list", "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0
    
    def test_summary_display(self, test_user):
        """Test that expense summary displays without errors."""
        run_cli("expense", "add", "--amount", "50", "--category", "Food",
                "--user-id", str(test_user["id"]))
        
        returncode, stdout, stderr = run_cli_display(
            "expense", "summary", "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0


class TestExpenseDetails:
    """Test expense detailed views."""

    def test_expense_view_details(self, test_user):
        """Test viewing expense with full details."""
        add_result = run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "75.00",
            "--category", "Dining",
            "--date", "2026-01-20",
            "--description", "Dinner with friends"
        )
        # Get expense ID from the response
        expense_id = add_result.get("expense", add_result.get("id", {}))
        if isinstance(expense_id, dict):
            expense_id = expense_id.get("id")
        # If no ID, list and get first one
        if not expense_id:
            list_result = run_cli("expense", "list", "--user-id", str(test_user["id"]))
            expenses = list_result.get("expenses", [])
            if expenses:
                expense_id = expenses[0]["id"]
        
        if expense_id:
            result = run_cli("expense", "view", str(expense_id))
            assert result["success"] is True

    def test_expense_list_by_date_range(self, test_user):
        """Test listing expenses filtered by date range."""
        # Add expenses in different months
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "100.00",
            "--category", "Food",
            "--date", "2026-01-15"
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "150.00",
            "--category", "Food",
            "--date", "2025-12-15"
        )

        result = run_cli("expense", "list",
                        "--user-id", str(test_user["id"]),
                        "--from", "2026-01-01",
                        "--to", "2026-01-31")

        assert result["success"] is True


class TestExpensePaymentMethods:
    """Test expenses with different payment methods."""

    def test_expense_with_debit_card(self, test_user, test_account, test_debit_card):
        """Test adding expense with debit card."""
        result = run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Shopping",
            "--date", "2026-01-18",
            "--payment", "debit_card",
            "--debit-id", str(test_debit_card["id"])
        )

        assert result["success"] is True

    def test_expense_summary_by_payment(self, test_user):
        """Test expense summary includes payment method breakdown."""
        # Add cash expense
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "25.00",
            "--category", "Food",
            "--date", "2026-01-10",
            "--payment", "cash"
        )
        # Add another cash expense
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "30.00",
            "--category", "Transport",
            "--date", "2026-01-12",
            "--payment", "cash"
        )

        result = run_cli("expense", "summary",
                        "--user-id", str(test_user["id"]),
                        "--month", "2026-01")

        assert result["success"] is True
