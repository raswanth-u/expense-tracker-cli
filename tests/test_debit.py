"""Test debit card CRUD operations."""

import pytest
from conftest import run_cli, run_cli_display


class TestDebitAdd:
    """Test debit add command."""
    
    def test_add_debit_card_basic(self, test_user, test_account):
        """Test adding a debit card linked to savings account."""
        result = run_cli(
            "debit", "add",
            "--user-id", str(test_user["id"]),
            "--name", "My Debit Card",
            "--last-four", "1234",
            "--account-id", str(test_account["id"])
        )
        
        assert result["success"] is True
        assert result["debit_card"]["card_name"] == "My Debit Card"
        assert result["debit_card"]["savings_account_id"] == test_account["id"]
    
    def test_add_debit_card_with_daily_limit(self, test_user, test_account):
        """Test adding a debit card with daily limit."""
        result = run_cli(
            "debit", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Limited Card",
            "--last-four", "5678",
            "--account-id", str(test_account["id"]),
            "--daily-limit", "500.00"
        )
        
        assert result["success"] is True
        assert float(result["debit_card"]["daily_limit"]) == 500.00
    
    def test_add_debit_card_with_tags(self, test_user, test_account):
        """Test adding a debit card with tags."""
        result = run_cli(
            "debit", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Tagged Card",
            "--last-four", "9012",
            "--account-id", str(test_account["id"]),
            "--tags", "primary,daily"
        )
        
        assert result["success"] is True
        assert result["debit_card"]["tags"] == "primary,daily"


class TestDebitList:
    """Test debit list command."""
    
    def test_list_empty(self, test_user):
        """Test listing when no debit cards exist."""
        result = run_cli("debit", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 0
    
    def test_list_debit_cards(self, test_user, test_account):
        """Test listing multiple debit cards."""
        # Create debit cards
        run_cli(
            "debit", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Debit Card 1",
            "--last-four", "1111",
            "--account-id", str(test_account["id"])
        )
        run_cli(
            "debit", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Debit Card 2",
            "--last-four", "2222",
            "--account-id", str(test_account["id"])
        )
        
        result = run_cli("debit", "list", "--user-id", str(test_user["id"]))
        
        assert result["success"] is True
        assert result["count"] == 2


class TestDebitView:
    """Test debit view command."""
    
    def test_view_debit_card(self, test_debit_card):
        """Test viewing a single debit card."""
        result = run_cli("debit", "view", str(test_debit_card["id"]))
        
        assert result["success"] is True
        assert result["debit_card"]["id"] == test_debit_card["id"]


class TestDebitUpdate:
    """Test debit update command."""
    
    def test_update_name(self, test_debit_card):
        """Test updating debit card name."""
        result = run_cli(
            "debit", "update", str(test_debit_card["id"]),
            "--name", "Updated Card Name"
        )
        
        assert result["success"] is True
        assert result["debit_card"]["card_name"] == "Updated Card Name"
    
    def test_update_daily_limit(self, test_debit_card):
        """Test updating daily limit."""
        result = run_cli(
            "debit", "update", str(test_debit_card["id"]),
            "--daily-limit", "1000.00"
        )
        
        assert result["success"] is True
        assert float(result["debit_card"]["daily_limit"]) == 1000.00


class TestDebitDelete:
    """Test debit delete command."""
    
    def test_delete_debit_card(self, test_user, test_account):
        """Test deleting a debit card."""
        add_result = run_cli(
            "debit", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Delete Me",
            "--last-four", "9999",
            "--account-id", str(test_account["id"])
        )
        card_id = add_result["id"]
        
        result = run_cli("debit", "delete", str(card_id), "--force")
        
        assert result["success"] is True
        
        # Verify card is gone
        list_result = run_cli("debit", "list", "--user-id", str(test_user["id"]))
        ids = [c["id"] for c in list_result["debit_cards"]]
        assert card_id not in ids


class TestDebitTransactions:
    """Test debit card transactions."""
    
    def test_list_transactions(self, test_debit_card):
        """Test listing transactions for a debit card."""
        result = run_cli("debit", "transactions", str(test_debit_card["id"]))
        
        assert result["success"] is True


class TestDebitAccountLinking:
    """Test debit card and savings account integration."""
    
    def test_expense_reduces_account_balance(self, test_user, test_debit_card, test_account):
        """Test that debit card expense reduces linked account balance."""
        # Get current balance from API
        account_before = run_cli("account", "view", str(test_account["id"]))
        initial_balance = account_before["summary"]["current_balance"]
        
        # Create expense with debit card
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "50.00",
            "--category", "Food",
            "--payment", "debit_card",
            "--debit-id", str(test_debit_card["id"])
        )
        
        # Check account balance
        account_result = run_cli("account", "view", str(test_account["id"]))
        new_balance = account_result["summary"]["current_balance"]
        
        assert new_balance == initial_balance - 50.00
    
    def test_multiple_expenses_cumulative(self, test_user, test_debit_card, test_account):
        """Test that multiple debit expenses are cumulative."""
        # Get current balance from API
        account_before = run_cli("account", "view", str(test_account["id"]))
        initial_balance = account_before["summary"]["current_balance"]
        
        # Create two expenses
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "30.00",
            "--category", "Food",
            "--payment", "debit_card",
            "--debit-id", str(test_debit_card["id"])
        )
        run_cli(
            "expense", "add",
            "--user-id", str(test_user["id"]),
            "--amount", "20.00",
            "--category", "Transport",
            "--payment", "debit_card",
            "--debit-id", str(test_debit_card["id"])
        )
        
        # Check account balance
        account_result = run_cli("account", "view", str(test_account["id"]))
        new_balance = account_result["summary"]["current_balance"]
        
        assert new_balance == initial_balance - 50.00


class TestDebitDisplay:
    """Test debit card display output (non-JSON)."""
    
    def test_list_display(self, test_user, test_account):
        """Test that debit list displays without errors."""
        run_cli(
            "debit", "add",
            "--user-id", str(test_user["id"]),
            "--name", "Display Card",
            "--last-four", "0000",
            "--account-id", str(test_account["id"])
        )
        
        returncode, stdout, stderr = run_cli_display(
            "debit", "list", "--user-id", str(test_user["id"])
        )
        
        assert returncode == 0
    
    def test_view_display(self, test_debit_card):
        """Test that debit view displays without errors."""
        returncode, stdout, stderr = run_cli_display(
            "debit", "view", str(test_debit_card["id"])
        )
        
        assert returncode == 0
